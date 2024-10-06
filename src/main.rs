use serde::Serialize;
use sysinfo::{System, SystemExt, DiskExt, ProcessExt, CpuExt, PidExt}; // Убедитесь, что импортируете необходимые структуры
use warp::Filter;
use std::time::Instant; // Импортируем для замера времени
use std::process::Command;
use std::str;
use std::collections::HashMap;

#[derive(Serialize)]
struct MemoryInfo {
    total: f64,     // Общая память в ГБ
    used: f64,      // Используемая память в ГБ
    available: f64, // Доступная память в ГБ
}

#[derive(Serialize)]
struct DiskDetails {
    name: String,
    total_space: f64, // Общий объем диска в ГБ
    used_space: f64,  // Используемый объем диска в ГБ
}

#[derive(Serialize)]
struct CpuDetails {
    id: usize,       // Номер процессора
    name: String,    // Название процессора
    frequency: u64,  // Частота процессора
}

#[derive(Serialize)]
struct VideoDetails {
    details: HashMap<String, HashMap<String, String>>, // Словарь для деталей видеоадаптера
}

#[derive(Serialize)]
struct DisplayDetails {
    details: HashMap<String, HashMap<String, String>>, // Словарь для деталей дисплея
}

#[derive(Serialize)]
struct ProcessDetails {
    pid: u32,            // Идентификатор процесса
    name: String,        // Название процесса
    status: String,      // Статус процесса
}

#[derive(Serialize)]
struct SystemInfo {
    memory: MemoryInfo,
    disks: Vec<DiskDetails>,
    cpu: Vec<CpuDetails>,
    video: Vec<VideoDetails>,      // Информация о видеоадаптерах
    displays: Vec<DisplayDetails>,  // Информация о дисплеях
    processes: Vec<ProcessDetails>,
    kernel_version: String, // Версия ядра
    execution_time: String,  // Время выполнения запроса
}

fn parse_video_info(output_str: &str) -> VideoDetails {
    let mut details = HashMap::new();
    let lines: Vec<&str> = output_str.lines().collect();
    let mut current_chipset = String::new();

    for line in lines {
        if line.contains("Chipset Model:") {
            current_chipset = line.replace("Chipset Model: ", "").trim().to_string();
            details.insert(current_chipset.clone(), HashMap::new());
        } else if current_chipset.is_empty() {
            continue; // Пропускаем строки до первого Chipset Model
        }

        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim().to_string();
            let value = line[pos + 1..].trim().to_string();
            details
                .get_mut(&current_chipset)
                .unwrap()
                .insert(key, value);
        }
    }

    VideoDetails { details }
}

fn parse_display_info(output_str: &str) -> DisplayDetails {
    let mut details = HashMap::new();
    let lines: Vec<&str> = output_str.lines().collect();
    let mut current_display = String::new();

    for line in lines {
        if line.contains("Display Type:") {
            current_display = line.replace("Display Type: ", "").trim().to_string();
            details.insert(current_display.clone(), HashMap::new());
        } else if current_display.is_empty() {
            continue; // Пропускаем строки до первого Display Type
        }

        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim().to_string();
            let value = line[pos + 1..].trim().to_string();
            details
                .get_mut(&current_display)
                .unwrap()
                .insert(key, value);
        }
    }

    DisplayDetails { details }
}

fn get_video_info() -> Vec<VideoDetails> {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");

    vec![parse_video_info(output_str)]
}

fn get_display_info() -> Vec<DisplayDetails> {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");

    vec![parse_display_info(output_str)]
}

#[tokio::main]
async fn main() {
    // Определяем маршрут для возврата системной информации в формате JSON
    let json_route = warp::path("system_info")
        .map(|| {
            let start_time = Instant::now(); // Начинаем замер времени

            let mut system = System::new_all();
            system.refresh_all(); // Обновляем данные системы

            // Собираем информацию о памяти
            let memory_info = MemoryInfo {
                total: system.total_memory() as f64 / (1024.0 * 1024.0),
                used: system.used_memory() as f64 / (1024.0 * 1024.0),
                available: system.free_memory() as f64 / (1024.0 * 1024.0),
            };

            // Собираем информацию о дисках
            let disks_info: Vec<DiskDetails> = system.disks().iter().map(|disk| {
                DiskDetails {
                    name: disk.name().to_string_lossy().into_owned(),
                    total_space: disk.total_space() as f64 / (1024.0 * 1024.0),
                    used_space: (disk.total_space() - disk.available_space()) as f64 / (1024.0 * 1024.0),
                }
            }).collect();

            // Собираем информацию о процессорах
            let cpu_info: Vec<CpuDetails> = system.cpus().iter().enumerate().map(|(id, cpu)| {
                CpuDetails {
                    id,
                    name: cpu.brand().to_string(),
                    frequency: cpu.frequency(),
                }
            }).collect();

            // Собираем информацию о видеоадаптерах
            let video_info = get_video_info();

            // Собираем информацию о дисплеях
            let displays_info = get_display_info();

            // Собираем информацию о процессах
            let processes_info: Vec<ProcessDetails> = system.processes().iter().map(|(&pid, process)| {
                ProcessDetails {
                    pid: pid.as_u32(), // Преобразуем Pid в u32
                    name: process.name().to_string(),
                    status: format!("{:?}", process.status()),
                }
            }).collect();

            // Заканчиваем замер времени
            let execution_time = format!("{:.2}", start_time.elapsed().as_secs_f64());

            // Создаем объект SystemInfo
            let system_info = SystemInfo {
                memory: memory_info,
                disks: disks_info,
                cpu: cpu_info,
                video: video_info, // Теперь здесь возвращаются детали без ключа `output`
                displays: displays_info, // Теперь здесь возвращаются детали без ключа `output`
                processes: processes_info,
                kernel_version: std::env::consts::OS.to_string(), // Версия операционной системы
                execution_time, // Время выполнения запроса
            };

            // Возвращаем системную информацию в формате JSON
            warp::reply::json(&system_info)
        });

    // Запускаем сервер на 127.0.0.1:8989
    warp::serve(json_route)
        .run(([127, 0, 0, 1], 8989))
        .await;
}