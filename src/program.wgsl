// 块大小
const block_size = 256;
alias Block = array<u32, 256>;

struct Lens {
    team_len: u32,
    work_count: u32,
    name_lens: array<u32>
}

/// 输入
// 统一的队名
@group(0) @binding(0) var<storage, read> team_name: array<u32>;
// 每个任务分别的队员名
@group(0) @binding(1) var<storage, read> names: array<Block>;
// 各种长度
@group(0) @binding(2) var<storage, read> lens: Lens;

/// 输出
@group(0) @binding(3) var<storage, read_write> result: array<Block>;

// 主函数
@compute @workgroup_size(16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // unpack4xU8
    let a = team_name[global_id.x % lens.team_len];
    let b = names[global_id.x / lens.team_len][global_id.x % lens.team_len];
    result[global_id.x][0] = 1u;
}