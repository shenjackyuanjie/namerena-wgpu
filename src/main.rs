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

fn main() -> Result<()>{
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;
    tokio_rt.block_on(async_main())?;
    Ok(())
}

async fn async_main() -> Result<()> {
    // 进行一个中文编程
    let 实例 = wgpu::Instance::default();

    let 适配器 = 实例.request_adapter(&wgpu::RequestAdapterOptions::default()).await.expect(" 无法找到适配器");

    let (设备, 队列) = 适配器.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("GPU"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
        },
        None,
    ).await?;

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
    
    let buffer = 设备.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("输入缓冲区"),
        contents: &[], // &[u8]``
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    });
    let bind_group_layout_0 = 计算管线.get_bind_group_layout(0);
    let bind_group_0 = 设备.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("绑定组"),
        layout: &bind_group_layout_0,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });

    Ok(())
}
