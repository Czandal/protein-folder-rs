use std::time::Duration;

use crate::{algorithm::compute_score::compute_score, config::Config};

use super::measure_time;

struct TestSuite<'a> {
    pub input_string: &'a str,
    pub max_expected_time: Duration,
}

struct TestSuiteOutput {
    pub duration: Duration,
    pub max_time_exceeded: bool,
    pub number_of_branches: usize,
    pub score: i32,
}

fn run_suite(suite: &TestSuite, config: &Config) -> TestSuiteOutput {
    let (duration, (score, number_of_branches)) =
        measure_time::measure_time(|| compute_score(suite.input_string, config.p1, config.p2));
    TestSuiteOutput {
        duration,
        max_time_exceeded: duration.gt(&suite.max_expected_time),
        score,
        number_of_branches,
    }
}

pub fn bench_mode<'a>(config: &Config) {
    let create_suite = |input: &'a str, millis: u64| -> TestSuite<'a> {
        TestSuite {
            input_string: input,
            max_expected_time: Duration::from_millis(millis),
        }
    };
    let suites_to_run: Vec<TestSuite> = vec![
        create_suite("HHHH", 4),
        create_suite("HPHPPHHPHPPHPHHPPHPH", 2500),
        create_suite("HHPPHPPHPPHPPHPPHPPHPPHH", 3500),
        create_suite("PPHPPHHPPPPHHPPPPHHPPPPHH", 4000),
        create_suite("PPPHHPPHHPPPPPHHHHHHHPPHHPPPPHHPPHPP", 10000),
        create_suite("PPHPPHHPPHHPPPPPHHHHHHHHHHPPPPPPHHPPHHPPHPPHHHHH", 16000),
        create_suite("PPHPPHPHPHHHHPHPPPHPPPHPPPPHPPPHPPPHPHHHHPHPHPHPHH", 30000),
        create_suite("PPHHHPHHHHHHHHPPPHHHHHHHHHHPHPPPHHHHHHHHHHHHPPPPHHHHHHPHHPHP", 35000),
        create_suite("HHHHHHHHHHHHPHPHPPHHPPHHPPHPPHHPPHHPPHPPHHPPHHPPHPHPHHHHHHHHHHHH", 36000),
        create_suite("HHHHPPPPHHHHHHHHHHHHPPPPPPHHHHHHHHHHHHPPPHHHHHHHHHHHHPPPHHHHHHHHHHHHPPPHPPHHPPHHPPHPH", 48000),
        create_suite("PPPHHPPHHHHPPHHHPHHPHHPHHHHPPPPPPPPHHHHHHPPHHHHHHPPPPPPPPPHPHHPHHHHHHHHHHHPPHHHPHHPHPPHPHHHPPPPPPHHH", 60000),
    ];
    let mut limit_exceeded = false;
    println!(
        "|  Input  |  Number of branches | Score | Time elapsed (ns) | Timeout exceeded | Ran |"
    );
    println!(
        "|---------|---------------------|-------|-------------------|------------------|-----|"
    );
    for suite in suites_to_run {
        if limit_exceeded {
            println!(
                "| {:} | {:} | {:} | {:} | {:} | {:} |",
                suite.input_string,
                "N/A",
                "N/A",
                "N/A",
                "N/A",
                false
            );
            continue;
        }
        let suite_output = run_suite(&suite, &config);
        println!(
            "| {:} | {:} | {:} | {:} | {:} | {:} |",
            suite.input_string,
            suite_output.number_of_branches,
            suite_output.score,
            suite_output.duration.as_nanos(),
            suite_output.max_time_exceeded,
            true
        );
        if suite_output.max_time_exceeded {
            limit_exceeded = true;
        }
    }
}
