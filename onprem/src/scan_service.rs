use std::borrow::Borrow;
use std::collections::HashSet;
use std::io::empty;
use walkdir::WalkDir;
use std::vec;
use std::{thread, time};
use tokio_util::sync::CancellationToken;

use super::on_prem::*;
use super::on_prem::scan_job::*;
use super::datasources::fileshare_datasource;

//TODO methods should probably return results instead
pub fn run<'a>(config: ServerConfig, token: CancellationToken) {
    while !token.is_cancelled() {
        //need some error handling in case it can't reach our servers...        

        let child_token = token.child_token();

        let job_response = check_for_job(JobRequest { company_id: config.company_id.clone() }, &child_token);

        let result = match job_response.job {
            Some(job_response::Job::ScanJob(scan_job)) => 
                match scan_job.job_type {
                    Some(JobType::ExploreRequest(request)) => {
                        handle_explore_job(request, child_token)
                    },
                    Some(JobType::AnalysisRequest(request)) => {
                        handle_analysis_job(request, child_token)
                    },
                    Some(JobType::CheckForDeletedFilesRequest(request)) => {
                        handle_check_for_deletion_job(request, child_token)
                    },
                    Some(JobType::DeleteFilesRequest(request)) => {
                        handle_delete_file_job(request, child_token)
                    }
                    //add all the other job types
                    _ => Err("didn't match any known job".to_string())
                }
            None => Ok("No new jobs to run".to_string()),
        };

        match result {
            Ok(message) => println!("Job completed with message: {}", message),
            Err(message) => println!("Job failed with message: {}", message)
        }

        thread::sleep(time::Duration::from_secs(config.sleep_interval_sec))
    }
}

fn check_for_job(request: JobRequest, token: &CancellationToken) -> JobResponse {
    //TODO ping server for job instead

    let current_dir = "/Users/martinholst/Desktop/";
    let valid_extensions: Vec<String> = vec!
    [
        "txt",
        "doc",
        "docx",
        "docm",
        "html",
        "pdf",
        "csv",
        "xls",
        "jpg",
        "jpeg",
        "png",
        "pptx"
    ].into_iter().map(|x| x.to_string()).collect();

    let t = 1;
    let job = match t {
        1 => JobType::ExploreRequest(ExploreRequest {
            root_path: current_dir.to_string(),
            valid_extensions:  valid_extensions,
        }),
        2 => JobType::AnalysisRequest(AnalysisRequest { 
            file_paths: vec!["", ""].into_iter().map(|x| x.to_string()).collect()
        }),
        3 => JobType::CheckForDeletedFilesRequest(CheckForDeletedFilesRequest {
            files: vec![
                File { 
                    id: "".to_string(), 
                    file_path: "/Users/martinholst/Desktop/Thesis_Files/results/experiment4.csv".to_string()
                }, File { 
                    id: "".to_string(), 
                    file_path: "/Users/martinholst/Desktop/Thesis_Files/results/experiment5.csv".to_string()
                }
            ]
        }),
        4 => JobType::DeleteFilesRequest(DeleteFilesRequest {
            files: vec![File { 
                id: "".to_string(), 
                file_path: "".to_string()
            }]
        }),
        _ => unimplemented!()
    };

    return JobResponse {
        job: Some(
            job_response::Job::ScanJob(ScanJob {
                company_id: request.company_id.to_owned(),
                service: ServiceType::FileShare.into(),
                job_type: Some(job)
            })
        )
    };
}

fn handle_explore_job(request: ExploreRequest, token: CancellationToken) -> Result<String, String> {
    let result = fileshare_datasource::explore_location(request, token);

    match result {
        Ok(response) => {
            for location in response.file_paths.iter() {
                println!("{}", location);
                //sends grpc requests with response
            }
        },
        Err(e) => return Err(e), //maybe send it 
    }
    Ok("Handle find locations completed".to_string())
}

fn handle_analysis_job(request: AnalysisRequest, token: CancellationToken) -> Result<String, String> {
    //determine based on service type which service to use

    //handle batch of files we need to analyze
    let result = fileshare_datasource::analyze_files(request, token);
    Ok("Analyzed files completed".to_string())
}

fn handle_check_for_deletion_job(request: CheckForDeletedFilesRequest, token: CancellationToken) -> Result<String, String> {
    let result = fileshare_datasource::check_for_deleted_files(request, token);

    match result {
        Ok(files_not_found) => println!("{}", files_not_found.len()), //send to api
        Err(err) => println!("{}", err)
    }

    Ok("Check for deleted files completed".to_string())
}

fn handle_delete_file_job<'a>(request: DeleteFilesRequest, token: CancellationToken) -> Result<String, String> {
    let result = fileshare_datasource::delete_files(request, token);
    Ok("delete files completed".to_string())
}

//TODO we need to handle the type of data source for each method either by switch or factory pattern