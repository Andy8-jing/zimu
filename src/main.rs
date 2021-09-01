extern crate serde_json;
use std::io::Write;
use std::env;
use std::fs;
use chrono::{TimeZone, Utc};
use glob::glob;
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
// use std::thread;

fn main(){

	// 接收启动参数
    let _args: Vec<String> = env::args().collect();

    if _args.len() < 2{
    	let stdout = stdout();
	    let message = String::from("\n剪映字幕json转vtt格式\n   作者：Andy.Jing\n\n1.批量转换剪映字幕到vtt格式\n  zimu '/path/*.json'\n\n2.单个文件转换\n  zimu /path/file_name.json\n\n");
	    let width = 300;

	    let mut writer = BufWriter::new(stdout.lock());
	    say(message.as_bytes(), width, &mut writer).unwrap();
    	
    }else{
	    println!("{}",_args[0]);
	    for entry in glob(&_args[1]).expect("Failed to read glob pattern") {
		    match entry {
		        Ok(path) => {
		        	get_vtt(path.display().to_string());
		        },
		        Err(e) => println!("{:?}", e),
		    }
		}
	}
}

fn get_vtt(arg: String){
	// 打开对应的json文件
    let f = fs::File::open(&arg).unwrap();

    // 拆分文件名和扩展名
    // let _file_name: Vec<&str> = arg.split('.').collect();
    let _file_name: &str = arg.trim_end_matches(".json");
    let _out_file_name: String = _file_name.to_string()+".vtt";

    // 读取文件内容
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();

    // 解析剪映json文件，获取字幕信息
    let materials = serde_json::json!(v["materials"]);
    let _tracks = serde_json::json!(v["tracks"]);

    // 字幕数据
    let _obj1 = &materials["texts"];
    // 字幕位置数据
  	let _obj2 = &_tracks[1];
  	// 字幕分片数量
	let _segments_len = &_obj2["segments"].as_array().unwrap().len();
	// 创建vtt字幕文件
	let mut file = fs::File::create(&_out_file_name).unwrap();

	// 闭包 传入秒为单位参数，返回两个结果 00:00:00,000
	let _get_time_str = |time: i64| {
		let _s_temp: i64  = time/1000;
		let _l_temp: i64 = time%1000;
		let mut l_temp = _l_temp.to_string();
		if _l_temp < 10{
			l_temp = "00".to_string() + &_l_temp.to_string();
		}else if _l_temp < 100{
			l_temp = "0".to_string() + &_l_temp.to_string();
		}else{
			l_temp = _l_temp.to_string();
		}
		let dt = Utc.timestamp(_s_temp, 0);
		let write_time = dt.format("%H:%M:%S").to_string();
		return (write_time,l_temp);
	};

	// 写文件头 WEBVTT 标识
	file.write("WEBVTT\n\n".as_bytes()).unwrap();

	for i in 0..*_segments_len{

		// 写入字幕编号
		file.write((i.to_string() + "\n").as_bytes()).unwrap();

		// 开始时间
		let start_time = serde_json::json!(_obj2["segments"][i]["target_timerange"]["start"]).as_i64().unwrap();
		let mut _start_time = start_time/1000;
		if _start_time > 999{
			let (_write_time,_l_temp) = _get_time_str(_start_time);
			file.write((_write_time + "." + &_l_temp.to_string() + " --> ").as_bytes()).unwrap();
		}else{
			file.write(("00:00:00".to_string() + "." + &_start_time.to_string() + " --> ").as_bytes()).unwrap();
		}

		// 结束时间
		let end_time = start_time + serde_json::json!(_obj2["segments"][i]["target_timerange"]["duration"]).as_i64().unwrap();
		let mut _end_time = end_time/1000;
		if _end_time > 999{
			let (_write_time,_l_temp) = _get_time_str(_end_time);
			file.write((_write_time + "." + &_l_temp.to_string() + "\n").as_bytes()).unwrap();
		}else{
			file.write(("00:00:00".to_string() + "." + &_end_time.to_string() + "\n").as_bytes()).unwrap();
		}

		let mut t_centent =  _obj1[i]["content"].to_string();

		//去除头尾双引号”“
		t_centent.pop();
		t_centent.remove(0);

		t_centent += "\n\n";

		file.write(t_centent.as_bytes()).unwrap();
	}
	println!("{} ======> ok",_out_file_name);
}