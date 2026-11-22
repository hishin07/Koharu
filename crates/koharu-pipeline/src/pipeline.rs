<<<<<<< HEAD
use std::sync::Arc;

use anyhow::{Context as _, Result};
use arc_swap::ArcSwap;
use koharu_config::Config;
use koharu_scene::{AssetRole, EntityId, Snapshot};

use crate::{
    Committer, PipelineConfig, PipelineError, Report, Request, ResourceSnapshot,
    execution::Execution, images::ImageCache, resources::ResourceMonitor,
    stage_runner::StageRunner,
=======
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
>>>>>>> upstream/main
};

use koharu_scene::{ProjectId, Session};
use parking_lot::{Mutex, RwLock};
use tokio::sync::{broadcast, mpsc};

use crate::{
    Error, Event, PipelineConfig, Request, Result, RunId,
    event::Events,
    scheduler::{Command, Plan, Run, Scheduler},
};

/// Runs Koharu's models for the app, its canvas, and the agent alike: a
/// cheap handle to clone. The scheduler and the thread owning the models end
/// when the last handle drops.
#[derive(Clone)]
<<<<<<< HEAD
pub struct Pipeline {
    current: Arc<ArcSwap<StageRunner>>,
    resources: Arc<ResourceMonitor>,
    images: Arc<ImageCache>,
    execution: Arc<tokio::sync::Mutex<()>>,
=======
pub struct Pipeline(Arc<Shared>);

struct Shared {
    config: Arc<RwLock<PipelineConfig>>,
    events: Events,
    commands: mpsc::UnboundedSender<Command>,
    next: AtomicU64,
>>>>>>> upstream/main
}

impl Pipeline {
    /// Starts the scheduler and the thread owning the models, so it needs a
    /// Tokio runtime. Models load on first use.
    #[must_use]
    pub fn new(config: PipelineConfig) -> Self {
        let config = Arc::new(RwLock::new(config));
        let events = Events::new();
        let commands = Scheduler::spawn(Arc::clone(&config), events.clone());
        Self(Arc::new(Shared {
            config,
            events,
            commands,
            next: AtomicU64::new(0),
        }))
    }

<<<<<<< HEAD
    #[tracing::instrument(skip_all)]
    pub fn from_config(
        config: Config<PipelineConfig>,
        providers: Config<koharu_translator::ProvidersConfig>,
        device: koharu_ml::Device,
    ) -> Result<Self> {
        let translator = koharu_translator::Translator::from_config(device.clone(), providers)?;
        let resources = ResourceMonitor::new(&device);
        let runner = {
            let value = config.read()?;
            StageRunner::new(&value, translator.clone(), &device, resources.clone())?
        };
        let current = Arc::new(ArcSwap::from_pointee(runner));
        let watched = current.clone();
        let watched_resources = resources.clone();
        let _watcher = tokio::runtime::Handle::try_current()
            .context("pipeline requires a Tokio runtime")?
            .spawn(async move {
                let mut changes = config.subscribe();
                while changes.changed().await.is_ok() {
                    let runner = config.read().and_then(|value| {
                        StageRunner::new(
                            &value,
                            translator.clone(),
                            &device,
                            watched_resources.clone(),
                        )
                    });
                    match runner {
                        Ok(runner) => watched.store(Arc::new(runner)),
                        Err(error) => tracing::error!(%error, "failed to reload pipeline"),
                    }
                }
            });
        Ok(Self {
            current,
            resources,
            images: Arc::new(ImageCache::default()),
            execution: Arc::new(tokio::sync::Mutex::new(())),
        })
    }

    /// Unloads every stage's model from this pipeline's current runner and
    /// releases Torch's allocator cache, so the freed VRAM is visible to any
    /// model loaded afterwards — including a fresh pipeline built while this
    /// one is still alive (its config watcher stays alive). Used by the
    /// `--verify` replay to make room for the second pass's models, and at
    /// every batch phase boundary so the next stage's model fits on small
    /// cards without waiting for an OOM to evict the previous one.
    pub fn unload_models(&self) {
        let runner = self.current.load_full();
        for stage in crate::Stage::ALL {
            runner.stages.unload(stage);
        }
        koharu_ml::torch_cache::empty();
    }

    /// Loads a stage's model outside every stage timer, so a phase's first
    /// page is billed warm: the first forward of the process pays runtime
    /// initialization that would otherwise land inside that page's stage
    /// duration. Best effort by contract — the model cell does not remember
    /// errors, so a failed preload lets the stage's own load retry exactly as
    /// before.
    pub async fn load_stage(&self, stage: crate::Stage) -> Result<()> {
        self.current.load_full().stages.load(stage).await
    }

    /// Decodes a page's `source` image into the pipeline's shared image cache
    /// without blocking the caller. The batch calls it for the pages after
    /// the one in flight, so that decode runs while the current page's stage
    /// occupies the models — the stage's own image `get` then hits the cache
    /// instead of serializing a decode. Best effort by contract — a failed or
    /// late prefetch only falls back to the stage's own decode.
    pub fn prefetch_image(&self, snapshot: Snapshot, page: EntityId) {
        let images = Arc::clone(&self.images);
        tokio::spawn(async move {
            let _ = images.get(&snapshot, page, "source").await;
        });
    }

    /// Folds a `source` image the caller already decoded into the shared
    /// image cache under the page's content-addressed blob. The batch calls
    /// it with the decode its page load just performed, so the stages reuse
    /// those pixels instead of reading and decoding the same bytes again for
    /// every stage of every phase. Best effort by contract — an unresolvable
    /// page or role only leaves the stage's own decode in place.
    pub(crate) fn insert_image(
        &self,
        snapshot: &Snapshot,
        page: EntityId,
        image: image::DynamicImage,
    ) {
        let Ok(role) = AssetRole::new("source") else {
            return;
        };
        let Ok(Some(asset)) = snapshot.asset(page, &role) else {
            return;
        };
        self.images.insert(asset.blob, Arc::new(image));
    }

    pub fn subscribe_resources(&self) -> tokio::sync::watch::Receiver<ResourceSnapshot> {
        self.resources.start();
        self.resources.subscribe()
    }

    #[tracing::instrument(skip_all)]
    pub async fn execute(
        &self,
        snapshot: Snapshot,
        request: Request,
        committer: &mut dyn Committer,
    ) -> std::result::Result<Report, PipelineError> {
        let _execution = self.execution.lock().await;
        Execution::new(
            self.current.load_full(),
            self.resources.clone(),
            self.images.clone(),
            snapshot,
            request,
            committer,
        )?
        .run()
        .await
=======
    /// Follows a changed `[pipeline]`. A model switched away from leaves
    /// memory once the work queued with it ends; runs already asked for keep
    /// the settings they started with.
    pub fn set_config(&self, config: PipelineConfig) {
        *self.0.config.write() = config;
        self.send(Command::Settle);
    }

    /// What every run tells the app from now on.
    #[must_use]
    pub fn events(&self) -> broadcast::Receiver<Event> {
        self.0.events.subscribe()
    }

    /// Queues `request` on the project `session` holds and returns its run.
    pub fn run(&self, session: Arc<Mutex<Session>>, request: Request) -> Result<RunId> {
        let config = request
            .config
            .unwrap_or_else(|| self.0.config.read().clone());
        let project = session.lock().project();
        let plan = Plan::new(&project, &request.target, &request.stages)?;
        let id = RunId(self.0.next.fetch_add(1, Ordering::Relaxed));
        // Told before the scheduler can tell anything of the run.
        self.0.events.send(Event::Queued {
            run: id,
            steps: plan.pages() * request.stages.len(),
        });
        let run = Run::new(id, session, project.id(), config, plan, &request.stages);
        self.send(Command::Run(run));
        Ok(id)
    }

    /// What `run` told so far, its events in order, once it ended or `wait`
    /// is over, whichever comes first; its last event says how it ended, as
    /// ComfyUI's `/history/{prompt_id}` keeps a prompt's.
    pub async fn history(&self, run: RunId, wait: Duration) -> Result<Vec<Event>> {
        self.0
            .events
            .history(run, wait)
            .await
            .ok_or_else(|| Error::invalid(format!("there is no run {}", run.0)))
    }

    /// What every run told so far, run by run, for a listener that came
    /// late, such as an interface that reloaded.
    #[must_use]
    pub fn told(&self) -> Vec<Event> {
        self.0.events.told()
    }

    /// Stops every run, running and queued. What they finished stays, and
    /// nothing more is written; a model call on the GPU finishes, but a
    /// local translation stops at its next token.
    pub fn stop(&self) {
        self.send(Command::Stop);
    }

    /// Unloads every model once the GPU's current work ends, as ComfyUI's
    /// Unload Models does.
    pub fn unload(&self) {
        self.send(Command::Unload);
    }

    /// Forgets the translation conversation of a project that closed.
    pub fn close(&self, project: ProjectId) {
        self.send(Command::Close(project));
    }

    fn send(&self, command: Command) {
        self.0
            .commands
            .send(command)
            .unwrap_or_else(|_| panic!("the scheduler runs while the pipeline does"));
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, time::Instant};

    use koharu_scene::Edit;
    use serde_json::json;

    use super::*;
    use crate::{Event, Outcome, Stage, Target};

    /// A project of one blank page, with no scan, so detection loads no
    /// model, in a folder removed with it.
    fn project() -> anyhow::Result<(tempfile::TempDir, Arc<Mutex<Session>>)> {
        let dir = tempfile::tempdir()?;
        let mut session = Session::create(dir.path().join("test.khr"))?;
        session.apply(Edit {
            label: "test".to_owned(),
            ops: vec![serde_json::from_value(json!({
                "op": "add_page", "index": 0, "size": {"width": 100, "height": 100},
            }))?],
        })?;
        Ok((dir, Arc::new(Mutex::new(session))))
    }

    #[tokio::test]
    async fn a_run_keeps_its_history_after_it_ended() {
        let (_dir, session) = project().expect("a project");
        let pipeline = Pipeline::new(PipelineConfig::default());
        let request = Request {
            target: Target::Project,
            stages: BTreeSet::from([Stage::Detect]),
            config: None,
        };
        let run = pipeline.run(session, request).expect("the run starts");
        let ended = Event::Ended {
            run,
            outcome: Outcome::Done,
        };
        for _ in 0..2 {
            let history = pipeline
                .history(run, Duration::from_secs(5))
                .await
                .expect("a history");
            assert_eq!(history.last(), Some(&ended));
        }
    }

    #[tokio::test]
    async fn an_unknown_run_fails_at_once() {
        let pipeline = Pipeline::new(PipelineConfig::default());
        let asked = Instant::now();
        let error = pipeline
            .history(RunId(999), Duration::from_secs(30))
            .await
            .expect_err("no such run");
        assert!(error.to_string().contains("999"), "{error}");
        assert!(asked.elapsed() < Duration::from_secs(1));
>>>>>>> upstream/main
    }
}
