use anyhow::Result;
use wgpu::{util::DeviceExt, BufferUsages};

// /// 传入数据
// pub struct WorkTeam {
//     /// 统一的队名 (讲道理也可以不相同, 但是没啥意义)
//     pub team: String,
//     /// 输入的队员
//     pub names: Vec<String>,
// }

const BLOCK_SIZE: usize = 256;

const PROGRAM_SOURCE: &str = include_str!("./program.wgsl");

fn main() -> Result<()> {
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;
    tokio_rt.block_on(async_main())?;
    Ok(())
}

type ShaderBlock = [u32; BLOCK_SIZE];

trait ShaderBlockExt {
    fn from_str(s: &str) -> Self;
    fn as_buffer(&self) -> &[u8];
}

impl ShaderBlockExt for ShaderBlock {
    fn from_str(s: &str) -> Self {
        let mut result = [0; BLOCK_SIZE];
        for (i, c) in s.chars().enumerate() {
            result[i] = c as u32;
        }
        result
    }
    fn as_buffer(&self) -> &[u8] {
        // u32 -> u8; 4 * n
        let ptr = self.as_ptr() as *const u8;
        unsafe { std::slice::from_raw_parts(ptr, BLOCK_SIZE * 4) }
    }
}

fn str_as_buffer(s: &str) -> &[u8] {
    // 先转换成 u32 数组
    let datas = s.chars().map(|c| c as u32).collect::<Vec<u32>>();
    // 再转换成 u8 数组
    let ptr = datas.as_ptr() as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, datas.len() * 4) }
}

// #[repr(C)]
// struct Lens<'a> {
//     team_len: u32,
//     work_count: u32,
//     name_lens: &'a [u32],
// }

// impl<'a> Lens<'a> {
//     pub fn as_buffer(&self) -> &[u8] {
//         let ptr = self as *const Self as *const u8;
//         unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of::<Self>()) }
//     }
//     pub fn new(team_len: u32, work_count: u32, name_lens: &'a [u32]) -> Self {
//         Self {
//             team_len,
//             work_count,
//             name_lens,
//         }
//     }
// }

async fn async_main() -> Result<()> {
    // 进行一个中文编程
    let 实例 = wgpu::Instance::default();

    let 适配器 = 实例
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect(" 无法找到适配器");

    let (设备, 队列) = 适配器
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("GPU"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )
        .await?;

    let cs_moudle = 设备.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("计算着色器"),
        source: wgpu::ShaderSource::Wgsl(PROGRAM_SOURCE.into()),
    });

    let pipline_descriptor = wgpu::ComputePipelineDescriptor {
        label: Some("管线描述"),
        cache: None,
        layout: None,
        module: &cs_moudle,
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        entry_point: "main",
    };
    let 计算管线 = 设备.create_compute_pipeline(&pipline_descriptor);

    let team_name = "x";
    let works = { vec!["x"; 1] };
    let work_count = works.len() as u32;
    // works as bytes
    let raw_works = works.iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
    let work_len_buffer = raw_works.iter().flat_map(|s| (s.len() as u32).to_ne_bytes()).collect::<Vec<u8>>();
    let filled_works = {
        let mut vecs = vec![];
        for work in works.iter() {
            vecs.extend_from_slice(ShaderBlock::from_str(work).as_buffer());
        }
        vecs
    };
    let lens = {
        let mut vecs = vec![];
        // team_len, work_count, name_lens
        vecs.extend_from_slice(&(team_name.len() as u32).to_ne_bytes());
        vecs.extend_from_slice(&(work_count as u32).to_ne_bytes());
        vecs.extend_from_slice(&work_len_buffer);
        vecs
    };

    let raw_data = str_as_buffer("x");

    let team_name_buffer = 设备.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("队名"),
        contents: raw_data, // &[u8]``
        usage: BufferUsages::STORAGE,
    });
    let names_buffer = 设备.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("队员"),
        contents: &filled_works, // &[u8]``
        usage: BufferUsages::STORAGE,
    });
    let lens_buffer = 设备.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("长度"),
        contents: &lens, // &[u8]``
        usage: BufferUsages::STORAGE,
    });
    let result_buffer = 设备.create_buffer(&wgpu::BufferDescriptor {
        label: Some("结果"),
        size: BLOCK_SIZE as u64 * 4,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind_group_layout_0 = 计算管线.get_bind_group_layout(0);
    let bind_group_0 = 设备.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("绑定组"),
        layout: &bind_group_layout_0,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: team_name_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: names_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: lens_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: result_buffer.as_entire_binding(),
            },
        ],
    });

    let limits = 设备.limits();
    println!("x轴最大工作组数: {}", limits.max_compute_workgroup_size_x);
    println!("y轴最大工作组数: {}", limits.max_compute_workgroup_size_y);
    println!("z轴最大工作组数: {}", limits.max_compute_workgroup_size_z);
    println!(
        "最大工作组数: {}",
        limits.max_compute_workgroups_per_dimension
    );

    let mut encoder = 设备.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        cpass.set_pipeline(&计算管线);
        cpass.set_bind_group(0, &bind_group_0, &[]);
        cpass.insert_debug_marker("计算");
        cpass.dispatch_workgroups(1, 1, 1);
    }

    println!("准备提交");
    队列.submit(Some(encoder.finish()));

    println!("提交完成");
    let buffer_slice = result_buffer.slice(..);
    let (sender, receiver) = tokio::sync::oneshot::channel::<()>();
    buffer_slice.map_async(wgpu::MapMode::Read, move |_| sender.send(()).unwrap());
    设备.poll(wgpu::Maintain::wait()).panic_on_timeout();
    Ok(())
}
