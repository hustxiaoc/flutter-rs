use std::sync::{Arc, Weak};

use flutter_engine::{
    channel::{MethodCallHandler, MethodChannel},
    codec::{value::Value as CodecValue, STANDARD_CODEC},
    plugins::Plugin,
    FlutterEngine,
};
use serde::{Deserialize, Serialize};
use flutter_engine::channel::MethodCall;
use parking_lot::Mutex;
use sqlite::{Connection, Value};
use std::collections::HashMap;
use serde_json::json;

const PLUGIN_NAME: &str = module_path!();
const CHANNEL_NAME: &str = "com.tekartik.sqflite";

pub trait SqfliteHandler {
    fn open(&mut self, db: String);
    fn query(&self, params: QueryDataBaseParams) -> serde_json::Value;
}

pub(crate) struct FlutterSqfliteHandler {
    databases_path: Option<String>,
    db_connection: Option<Connection>,
}

fn create_codec_value(v: &Value) -> CodecValue {
    match v {
        Value::Float(v) => CodecValue::F64(*v),
        Value::Integer(v) => CodecValue::I64(*v),
        Value::String(v) => CodecValue::String(v.to_string()),
        _ => CodecValue::Null,
    }
}

fn to_sql_value(v: &CodecValue) -> Value {
    match v {
        CodecValue::F64(v) => Value::Float(*v),
        CodecValue::I64(v) => Value::Integer(*v),
        CodecValue::String(v) => Value::String(v.to_string()),
        _ => Value::Null,
    }
}

impl FlutterSqfliteHandler {
    pub fn new() -> Self {
        Self {
            databases_path: None,
            db_connection: None,
        }
    }
}

impl SqfliteHandler for FlutterSqfliteHandler {
    fn open(&mut self, db: String) {
        if self.db_connection.is_none() {
            let connection = sqlite::open(db).unwrap();
            self.db_connection = Some(connection);
        }
    }

    fn query(&self, params: QueryDataBaseParams) -> serde_json::Value {
        println!("query database {:?}", params.sql);

        let connection = self.db_connection.as_ref().unwrap();
        let mut statement = connection.prepare(params.sql).unwrap();

        if params.arguments.is_some() {
            for (pos, ele) in params.arguments.as_ref().unwrap().iter().enumerate() {
                statement.bind(pos + 1, &to_sql_value(ele)).unwrap();
            }
        }

        let columns = statement.names().iter().map(|v| v.to_string()).collect::<Vec<String>>();

        let mut rows = vec![];
        let mut cursor = statement.cursor();

        while let Some(row) = cursor.next().unwrap() {
            let val: Vec<CodecValue> = row.iter().map(|v|{
                create_codec_value(v)
            }).collect();
            rows.push(val);
        }

        json!({
            "rows": rows,
            "columns": columns,
        })
    }
}

pub struct SqflitePlugin {
    channel: Weak<MethodChannel>,
    handler: Arc<Mutex<dyn SqfliteHandler + Send>>,
}

impl SqflitePlugin {
    pub fn new() -> Self {
        Self {
            channel: Weak::new(),
            handler: Arc::new(Mutex::new(FlutterSqfliteHandler::new())),
        }
    }
}

impl Plugin for SqflitePlugin {
    fn plugin_name() -> &'static str {
        PLUGIN_NAME
    }

    fn init(&mut self, engine: &FlutterEngine) {
        self.channel = engine.register_channel(MethodChannel::new(
            CHANNEL_NAME,
            Handler {
                handler: self.handler.clone(),
            },
            &STANDARD_CODEC,
        ));
    }
}

struct Handler {
    handler: Arc<Mutex<dyn SqfliteHandler + Send>>,
}

impl MethodCallHandler for Handler {
    fn on_method_call(&mut self, call: MethodCall) {
        match call.method().as_str() {
            "getDatabasesPath" => {
                call.success("");
            }
            "openDatabase" => {
                let args: OpenDataBaseParams = call.args();
                self.handler.lock().open(args.path);
                let mut result = HashMap::new();
                result.insert("databaseId", "");
                call.success(result);
            },
            "query" => {
                let args: QueryDataBaseParams = call.args();
                println!("on_method_call {:?} {:?}", call.method().as_str(), args);
                let result = self.handler.lock().query(args);
                // println!("result is {:?}", result);
                call.success(result);
            }
            _ => {
                println!("on_method_call {:?}", call.method().as_str());
                call.success_empty();
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenDataBaseParams {
    pub path: String,
    pub readOnly: Option<bool>,
    pub singleInstance: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryDataBaseParams {
    pub sql: String,
    pub arguments: Option<Vec<CodecValue>>
}
