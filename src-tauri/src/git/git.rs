use std::process::Command;
use crate::types::{error::Error, git_job::GitJob};

fn run_jobs(job: GitJob) -> Result<(),Error> {

    let final_release_branch = format!("origin/{}", job.version);

    for repository in job.repositories {

    }
}


