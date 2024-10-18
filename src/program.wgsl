
const block_size = 256;

struct String {
    data: array<u32, block_size>,
}

struct WorkTeam {
    // 奢侈点, 直接 u32 当 u8 用
    team: array<u32, block_size>,
    // 一堆名字
    names: array<String>,
}

// 输入
@group(0) @binding(0) var<storage, read_write> team: WorkTeam;

// 输出
@group(0) @binding(1) var result: array<array<u32, block_size>>;

// 主函数
@compute @workgroup_size(16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

}