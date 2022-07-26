/*
Copyright 2022 The Workpieces LLC.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Work {
    pub id: i32,
    pub work_code: String,
    pub add_up_to: i32,
    pub done: bool,
}

fn main() {
    // 结构体转string
    let w = Work {
        id: 1,
        work_code: "foo".to_string(),
        add_up_to: 100,
        done: false,
    };

    let json = serde_json::to_string(&w).unwrap();
    println!("{}", json);

    // string转结构体
    let body = r#"
        {
            "id": 100,
            "work_code": "Running",
            "add_up_to": 200,
            "done": :false,
        }"#;

    let v: Work = serde_json::from_str(body).unwrap();
    println!("Work Code: {}", v.work_code);
}
