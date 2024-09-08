pub mod user_jobs{

    #[derive(Debug)]
    pub struct JobParams {
        pub path: String,

    }

    #[derive(Debug)]
    pub struct Job {
        pub name: String,
        pub timing: String,
        pub params: JobParams,
    }
}
