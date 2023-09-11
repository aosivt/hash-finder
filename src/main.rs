//! Главный модуль расчета колекции хеш значений
use std::{env, error::Error, sync::{atomic::{AtomicI32, Ordering}, Arc, Mutex}, thread};

use sha2::{Sha256, Digest};
use struct_hasher::{error_hasher::WrongArgs, opt_args::OptArgsHasher};
use log::{error, info};

pub mod struct_hasher;
mod logs;

const NAME_N: &str = "-N";
const NAME_F: &str = "-F";
const DEFAULT_DIGIT: i32 = 1;
fn main() {
    logs::logs::init();
    
    let threads = num_cpus::get();
    let opt = OptArgsHasher::init();
    let template_end_with = generate_template_end_with(&opt);

    info!("Приложение запущено со следубщими параметрами: количество ядер: {}, хешь заканчивается на {}, количество необходимых результатов {}", threads, template_end_with.clone(), opt.count_result);
    research(threads, template_end_with, opt)

}
/** Функция расчета коллекции 
 * 
*/
fn research(threads:usize, template_end_with: String, opt: OptArgsHasher){
    let  results: Vec<String> = Vec::new();
    let _counter = AtomicI32::new(DEFAULT_DIGIT);
    
    let shared_counter = Arc::new(Mutex::new(_counter));
    let shared_result = Arc::new(Mutex::new(results));
    let shared_template_end_with = Arc::new(Mutex::new(template_end_with));

    for _ in 0..threads {

        let inside_shared_counter = shared_counter.clone();
        let inside_shared_result = shared_result.clone();
        let inside_shared_template_end_with = shared_template_end_with.clone();
        
        thread::spawn(move || {
            let _template_end_with = inside_shared_template_end_with.lock().unwrap().clone();
            
            loop{
                let mut shared = inside_shared_counter.lock().unwrap();
                let mut _results = inside_shared_result.lock().unwrap();
                *shared.get_mut() = shared.get_mut().abs() + 1;
                let result = generate_string(shared.get_mut().abs());
                if result.ends_with(&_template_end_with){
                    _results.push(result);
                }
                
            }
        });
    }

    loop {
        let _results = shared_result.lock().unwrap().clone();
        if _results.len() > (opt.count_result - 1) as usize {
            _results.iter().for_each(|r| println!("{}",r));
            break;
        }
    }

}

fn generate_template_end_with(opt: &OptArgsHasher) -> String{
    let vec = vec![0; opt.count_zero as usize];
    vec.into_iter().map(|d| d.to_string()).collect::<String>()
}

fn generate_string(digit: i32) -> String{
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(digit.to_string());
    format!("{:X}", hasher.finalize())
}

#[test]
fn fielded_template(){
    let opt = OptArgsHasher{count_result: 2, count_zero: 2};
    let result = generate_template_end_with(&opt);
    assert_eq!(result,"00".to_string());
    assert!( "12300".to_string().ends_with(&result));
}