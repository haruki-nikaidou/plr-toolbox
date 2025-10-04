use kanau::processor::Processor;
use time::PrimitiveDateTime;

#[derive(Debug, Clone)]
pub struct JobCompleteSignal<Id> {
    pub id: Id,
    pub complete_time: PrimitiveDateTime,
}

/// Type alias for job execution result
pub type JobResult<Id> = Result<JobCompleteSignal<Id>, crate::Error>;

pub trait OneGoScheduledJob: Sized {
    /// After each `CLOCK_LOOPS` intervals, the job will be executed
    const CLOCK_LOOPS: u64 = 1;
    type Executor: Processor<Self, JobResult<()>> + Send;
    type Scanner: Processor<PrimitiveDateTime, Result<Self, crate::Error>> + Send;
}

pub async fn cron_one_go<T: OneGoScheduledJob>(
    scanner: &T::Scanner,
    executor: &T::Executor,
    now: PrimitiveDateTime,
) -> Result<(), crate::Error> {
    let jobs = scanner.process(now).await?;
    executor.process(jobs).await?;
    Ok(())
}