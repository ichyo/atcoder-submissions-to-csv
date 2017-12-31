extern crate chrono;
extern crate regex;
extern crate scraper;

use chrono::prelude::*;
use scraper::{Html, Selector};
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Submission {
    created: DateTime<Local>,
    task: String,
    user: String,
    language: String,
    score: i32,
    code_size: i32,
    status: String,
    exec_time_ms: Option<i32>,
    memory_kb: Option<i32>,
}

impl Submission {
    fn from_html(html: &str) -> Vec<Submission> {
        let body_row_selector = Selector::parse("table tbody tr").unwrap();
        let data_selector = Selector::parse("td").unwrap();
        let a_selector = Selector::parse("a").unwrap();

        let fragment = Html::parse_fragment(html);

        let mut res = Vec::new();
        for row_element in fragment.select(&body_row_selector) {
            let mut iter = row_element.select(&data_selector);
            let created = DateTime::parse_from_str(
                iter.next().unwrap().text().next().unwrap().trim(),
                "%Y-%m-%d %H:%M:%S%z",
            ).unwrap()
                .with_timezone(&Local {});
            let task = iter.next()
                .unwrap()
                .select(&a_selector)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .trim()
                .split("/")
                .last()
                .unwrap();
            let user = iter.next().unwrap().text().collect::<Vec<_>>().concat();
            let language = iter.next().unwrap().text().collect::<Vec<_>>().concat();
            let score = iter.next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .concat()
                .parse::<i32>()
                .unwrap();
            let code_size = iter.next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .concat()
                .split(" ")
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let status_elem = iter.next().unwrap();
            let status = status_elem.text().collect::<Vec<_>>().concat();
            let (exec_time_ms, memory_kb) = match status_elem.value().attr("colspan") {
                Some("3") => (None, None),
                _ => (
                    Some(
                        iter.next()
                            .unwrap()
                            .text()
                            .collect::<Vec<_>>()
                            .concat()
                            .split(" ")
                            .next()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap(),
                    ),
                    Some(
                        iter.next()
                            .unwrap()
                            .text()
                            .collect::<Vec<_>>()
                            .concat()
                            .split(" ")
                            .next()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap(),
                    ),
                ),
            };
            res.push(Submission {
                created: created,
                task: task.trim().to_owned(),
                user: user.trim().to_owned(),
                language: language.trim().to_owned(),
                score: score,
                code_size: code_size,
                status: status.trim().to_owned(),
                exec_time_ms: exec_time_ms,
                memory_kb: memory_kb,
            });
        }
        res
    }
}

fn get_submissions_url(contest_id: &str, page: u32) -> String {
    format!(
        "https://beta.atcoder.jp/contests/{contest_id}/submissions?orderBy=created&page={page}",
        contest_id = contest_id,
        page = page + 1
    )
}

fn fetch_and_parse_submission_page(contest_id: &str, page: u32) -> Vec<Submission> {
    vec![]
}

pub fn fetch_submissions(contest_id: &str, start: u32, limit: u32) -> Vec<Submission> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_submissions_url() {
        assert_eq!(
            "https://beta.atcoder.jp/contests/arc088/submissions?orderBy=created&page=1",
            get_submissions_url("arc088", 0)
        )
    }

    #[test]
    fn test_from_html() {
        assert_eq!(vec![
           Submission {
               created: FixedOffset::east(9 * 3600).ymd(2017, 12, 23).and_hms(21, 06, 26).with_timezone(&Local{}),
               task: "arc088_a".to_owned(),
               user: "ichyo".to_owned(),
               language: "Rust (1.15.1)".to_owned(),
               score: 300,
               code_size: 1768,
               status: "AC".to_owned(),
               exec_time_ms: Some(2),
               memory_kb: Some(4352),
           },
           Submission {
               created: FixedOffset::east(9 * 3600).ymd(2017, 12, 23).and_hms(21, 27, 21).with_timezone(&Local{}),
               task: "arc088_b".to_owned(),
               user: "ichyo".to_owned(),
               language: "Rust (1.15.1)".to_owned(),
               score: 0,
               code_size: 2094,
               status: "RE".to_owned(),
               exec_time_ms: None,
               memory_kb: None,
           },
           Submission {
               created: FixedOffset::east(9 * 3600).ymd(2017, 12, 23).and_hms(21, 32, 03).with_timezone(&Local{}),
               task: "arc088_b".to_owned(),
               user: "ichyo".to_owned(),
               language: "Rust (1.15.1)".to_owned(),
               score: 0,
               code_size: 2145,
               status: "WA".to_owned(),
               exec_time_ms: Some(2),
               memory_kb: Some(4352),
           },
        ],
        Submission::from_html(
            r#"
<div class="table-responsive">
    <table class="table table-bordered table-striped small th-center">
        <thead>
        <tr>

            <th width="12%"><a href='/contests/arc088/submissions/me?desc=true&amp;orderBy=created'>Submission Time</a></th>
            <th>Task</th>
            <th>User</th>
            <th>Language</th>
            <th width="5%"><a href='/contests/arc088/submissions/me?desc=true&amp;orderBy=score'>Score</a></th>
            <th width="9%"><a href='/contests/arc088/submissions/me?orderBy=source_length'>Code Size</a></th>
            <th width="5%">Status</th>
            <th width="7%"><a href='/contests/arc088/submissions/me?orderBy=time_consumption'>Exec Time</a></th>
            <th width="8%"><a href='/contests/arc088/submissions/me?orderBy=memory_consumption'>Memory</a></th>
            <th width="5%"></th>
        </tr>
        </thead>
        <tbody>

            <tr>

                <td class="no-break"><time class='fixtime fixtime-second'>2017-12-23 21:06:26+0900</time></td>
                <td><a href='/contests/arc088/tasks/arc088_a'>C - Multiple Gift</a></td>
                <td><a href='/users/ichyo'>ichyo</a> <a href='/contests/arc088/submissions?f.User=ichyo'><span class='glyphicon glyphicon-search black' aria-hidden='true' data-toggle='tooltip' title='view ichyo's submissions'></span></a></td>
                <td>Rust (1.15.1)</td>
                <td class="text-right submission-score" data-id="1896399">300</td>
                <td class="text-right">1768 Byte</td>
                <td class='text-center'><span class='label label-success' aria-hidden='true' data-toggle='tooltip' data-placement='top' title="Accepted">AC</span></td><td class='text-right'>2 ms</td><td class='text-right'>4352 KB</td>
                <td class="text-center">
                    <a href='/contests/arc088/submissions/1896399'>Detail</a>
                </td>
            </tr>

            <tr>

                <td class="no-break"><time class='fixtime fixtime-second'>2017-12-23 21:27:21+0900</time></td>
                <td><a href='/contests/arc088/tasks/arc088_b'>D - Wide Flip</a></td>
                <td><a href='/users/ichyo'>ichyo</a> <a href='/contests/arc088/submissions?f.User=ichyo'><span class='glyphicon glyphicon-search black' aria-hidden='true' data-toggle='tooltip' title='view ichyo's submissions'></span></a></td>
                <td>Rust (1.15.1)</td>
                <td class="text-right submission-score" data-id="1898069">0</td>
                <td class="text-right">2094 Byte</td>
                <td colspan='3' class='text-center'><span class='label label-warning' aria-hidden='true' data-toggle='tooltip' data-placement='top' title="Runtime Error">RE</span></td>
                <td class="text-center">
                    <a href='/contests/arc088/submissions/1898069'>Detail</a>
                </td>
            </tr>

            <tr>

                <td class="no-break"><time class='fixtime fixtime-second'>2017-12-23 21:32:03+0900</time></td>
                <td><a href='/contests/arc088/tasks/arc088_b'>D - Wide Flip</a></td>
                <td><a href='/users/ichyo'>ichyo</a> <a href='/contests/arc088/submissions?f.User=ichyo'><span class='glyphicon glyphicon-search black' aria-hidden='true' data-toggle='tooltip' title='view ichyo's submissions'></span></a></td>
                <td>Rust (1.15.1)</td>
                <td class="text-right submission-score" data-id="1898306">0</td>
                <td class="text-right">2145 Byte</td>
                <td class='text-center'><span class='label label-warning' aria-hidden='true' data-toggle='tooltip' data-placement='top' title="Wrong Answer">WA</span></td><td class='text-right'>2 ms</td><td class='text-right'>4352 KB</td>
                <td class="text-center">
                    <a href='/contests/arc088/submissions/1898306'>Detail</a>
                </td>
            </tr>

        </tbody>
    </table>
</div>
"#));
    }
}
