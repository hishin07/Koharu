'use client'

import { useMutationState } from '@tanstack/react-query'
import { CircleAlert, Download, LoaderCircle, Square, X } from 'lucide-react'
<<<<<<< HEAD
import { useState } from 'react'
=======
import type { ReactNode } from 'react'
>>>>>>> upstream/main
import { useTranslation } from 'react-i18next'

import { send, usePages } from '@/lib/queries'
import { useKoharuStore, type Job } from '@/lib/store'
import { commands, type Transfer } from '@koharu/ipc/protocol'
import { Button } from '@koharu/ui/components/button'
import { Progress, ProgressLabel, ProgressValue } from '@koharu/ui/components/progress'

export function ActivityCenter() {
  const { t } = useTranslation()
  const jobs = useKoharuStore((state) => state.jobs)
  const downloads = useKoharuStore((state) => state.downloads)
  const activities = useMutationState({
    filters: { status: 'pending' },
    select: (mutation) => ({ id: mutation.mutationId, label: mutation.meta?.activity }),
  }).flatMap(({ id, label }) => (label ? [{ id, label }] : []))
  const visibleJobs = Object.values(jobs).filter(
    (job) => job.state === 'running' || job.state === 'failed',
  )
  const runningDownloads = Object.values(downloads).filter(
    (download) => download.state === 'Running',
  )
  const failedDownloads = Object.values(downloads).filter((download) => failure(download) !== null)
  const visibleDownloads = [...runningDownloads, ...failedDownloads]
  if (activities.length === 0 && visibleJobs.length === 0 && visibleDownloads.length === 0)
    return null

  return (
    <aside
      aria-label={t('activity.title')}
      className='absolute right-3 bottom-9 z-30 flex w-72 max-w-[calc(100%-1.5rem)] flex-col rounded-2xl border border-border/50 bg-popover shadow-md'
    >
      <div className='border-b px-3 py-2'>
        <span className='text-[10px] font-semibold tracking-[0.1em] uppercase'>
          {t('activity.title')}
        </span>
      </div>
      {activities.map((activity) => (
        <div
          key={activity.id}
          role='status'
          className='grid grid-cols-[1rem_minmax(0,1fr)] items-center gap-x-2.5 border-b p-3 last:border-b-0'
        >
          <LoaderCircle
            className='size-3.5 animate-spin justify-self-center text-primary motion-reduce:animate-none'
            aria-hidden='true'
          />
          <span className='truncate text-[11px] font-medium'>{t(activity.label)}</span>
        </div>
      ))}
      {runningDownloads.length > 1 ? (
        <DownloadGroup downloads={runningDownloads} />
      ) : (
        runningDownloads.map((download) => <DownloadItem key={download.id} download={download} />)
      )}
      {failedDownloads.map((download) => (
        <DownloadItem key={download.id} download={download} />
      ))}
      {visibleJobs.map((job) => (
        <JobItem key={job.id} job={job} />
      ))}
    </aside>
  )
}

function DownloadGroup({ downloads }: { downloads: Transfer[] }) {
  const { t } = useTranslation()
  const total = downloads.reduce((sum, download) => sum + download.total, 0)
  const completed = downloads.reduce((sum, download) => sum + download.completed, 0)
  return (
    <ProgressRow
      icon={<Download className='mt-0.5 size-3.5 justify-self-center text-primary' />}
      label={t('activity.downloadingFiles', { count: downloads.length })}
      completed={downloads.every((download) => download.total > 0) ? completed : null}
      total={total}
    />
  )
}

function JobItem({ job }: { job: Job }) {
  const { t } = useTranslation()
  const dismiss = useKoharuStore((state) => state.dismissJob)
  // Numbered by position in the rail, so the row says where in the run this is.
  const pages = usePages(job.page !== null).data
  const pageIndex = job.page ? (pages?.findIndex((page) => page.id === job.page) ?? -1) : -1
  const pageLabel =
    pageIndex >= 0 ? t('navigator.pageNumber', { number: pageIndex + 1 }) : undefined
  if (job.state === 'failed') {
    return (
      <Failure
        message={job.error || t('activity.processingFailed')}
        detail={pageLabel}
        onDismiss={() => dismiss(String(job.id))}
      />
    )
  }
  return (
    <ProgressRow
      icon={<span className='mt-1.5 size-1.5 justify-self-center rounded-full bg-primary' />}
      label={
        job.stage ? t(`phase.${job.stage}`, { defaultValue: job.stage }) : t('activity.processing')
      }
      detail={pageLabel}
      completed={job.completed}
      total={job.total}
      action={
        <Button
          size='icon-xs'
          variant='ghost'
          className='-mt-1'
          aria-label={t('activity.stop')}
          onClick={() => send(commands.stopPipeline)}
        >
          <Square className='size-2.5 fill-current' />
        </Button>
      }
    />
  )
}

function DownloadItem({ download }: { download: Transfer }) {
  const { t } = useTranslation()
  const dismiss = useKoharuStore((state) => state.dismissDownload)
  const error = failure(download)
  if (error !== null) {
    return (
      <Failure
        message={error || t('activity.downloadFailed')}
        onDismiss={() => dismiss(download.id)}
      />
    )
  }
  return (
    <ProgressRow
      icon={<Download className='mt-0.5 size-3.5 justify-self-center text-primary' />}
      label={download.name || t('activity.modelDownload')}
      completed={download.completed}
      total={download.total}
    />
  )
}

/** A row of work under way: its name and how far along it is, in the interface's language, over
 * its bar, which runs on its own while the total is unknown. */
function ProgressRow({
  icon,
  label,
  detail,
  completed,
  total,
  action,
}: {
  icon: ReactNode
  label: string
  /** Where the work is, under its name. */
  detail?: string
  completed: number | null
  total: number
  action?: ReactNode
}) {
  const { i18n } = useTranslation()
  return (
    <div className='grid grid-cols-[1rem_minmax(0,1fr)_1.5rem] items-start gap-x-2.5 border-b p-3 last:border-b-0'>
      {icon}
      <Progress
        value={completed !== null && total > 0 ? Math.min(completed, total) : null}
        max={total || 100}
        locale={i18n.language}
        className='min-w-0 gap-x-2 gap-y-1.5'
      >
        <ProgressLabel className='min-w-0 flex-1 truncate text-[11px] font-medium'>
          {label}
        </ProgressLabel>
        <ProgressValue className='text-[10px] text-foreground' />
        {detail && <p className='w-full truncate text-[10px] text-muted-foreground'>{detail}</p>}
      </Progress>
      {action ?? <span aria-hidden='true' />}
    </div>
  )
}

<<<<<<< HEAD
const PREVIEW_HEAD = 96
const PREVIEW_TAIL = 64
/// A longer failure line is previewed as head + ellipsis + tail.
const PREVIEW_LIMIT = PREVIEW_HEAD + PREVIEW_TAIL

/// The GGUF file named in a message line, without its directory separators.
const GGUF_RE = /[^\\/\s]*\.gguf/gi

interface Preview {
  text: string
  /// The GGUF filename inside `text`, kept whole so the failing model stays
  /// identifiable; `null` when the line names no model.
  filename: string | null
}

/// Builds a compact preview of a failure: the first line only, with the middle
/// replaced by an ellipsis when that line is long (e.g. a full GGUF path). The
/// GGUF filename stays whole so the ellipsis never cuts it, and the following
/// lines (llama.cpp's captured logs) stay behind the expand toggle.
function previewOf(message: string): Preview {
  const line = message.split('\n')[0] ?? ''
  const filename = [...line.matchAll(GGUF_RE)].at(-1)?.[0] ?? null
  if (line.length <= PREVIEW_LIMIT) return { text: line, filename }
  // The default tail is the last PREVIEW_TAIL characters; when it would cut
  // the GGUF filename, the tail starts at the filename instead.
  let tailStart = line.length - PREVIEW_TAIL
  const fileStart = filename ? line.lastIndexOf(filename) : -1
  if (filename && fileStart > PREVIEW_HEAD && fileStart + filename.length > tailStart) {
    tailStart = fileStart
  }
  return { text: `${line.slice(0, PREVIEW_HEAD)}…${line.slice(tailStart)}`, filename }
}

/// Renders the preview with the GGUF filename emphasized.
function PreviewText({ text, filename }: Preview) {
  const at = filename ? text.indexOf(filename) : -1
  if (filename === null || at < 0) return text
  return (
    <>
      {text.slice(0, at)}
      <span className='font-semibold text-foreground'>{filename}</span>
      {text.slice(at + filename.length)}
    </>
  )
}

function Failure({ message, onDismiss }: { message: string; onDismiss: () => void }) {
=======
/** A failure's row: why, and under it where, as a running row names its page. */
function Failure({
  message,
  detail,
  onDismiss,
}: {
  message: string
  detail?: string
  onDismiss: () => void
}) {
>>>>>>> upstream/main
  const { t } = useTranslation()
  const [expanded, setExpanded] = useState(false)
  const preview = previewOf(message)
  const hasMore = preview.text !== message
  return (
    <div className='grid grid-cols-[1rem_minmax(0,1fr)_2.25rem_1.5rem] items-start gap-x-2.5 border-b p-3 text-[11px] last:border-b-0'>
      <CircleAlert className='mt-0.5 size-3.5 justify-self-center text-destructive' />
      <div className='col-start-2 col-end-4 min-w-0'>
<<<<<<< HEAD
        {expanded ? (
          <pre className='max-h-48 overflow-y-auto font-mono text-[10px] leading-4 break-words whitespace-pre-wrap text-destructive'>
            {message}
          </pre>
        ) : (
          <span className='block break-words text-destructive'><PreviewText {...preview} /></span>
        )}
        {hasMore && (
          <button
            type='button'
            aria-expanded={expanded}
            className='mt-1 block font-medium text-destructive/80 hover:text-destructive'
            onClick={() => setExpanded((value) => !value)}
          >
            {expanded ? t('activity.showLess') : t('activity.showMore')}
          </button>
        )}
=======
        <p className='text-destructive'>{message}</p>
        {detail ? (
          <p className='mt-0.5 truncate text-[10px] text-muted-foreground'>{detail}</p>
        ) : null}
>>>>>>> upstream/main
      </div>
      <Button
        size='icon-xs'
        variant='ghost'
        className='-mt-1'
        aria-label={t('activity.dismiss')}
        onClick={onDismiss}
      >
        <X />
      </Button>
    </div>
  )
}

/** Why a download failed, or `null` while it hasn't. */
function failure(download: Transfer): string | null {
  return typeof download.state === 'object' ? download.state.Failed : null
}
