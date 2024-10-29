// 块大小
const block_size = 256;
// 打包成 u32 的 u8 数组
// 需要使用 unpack4xU8 解包
alias U8BLock = array<u32, 64>;
// 原始的 u8 数组
// 因为 wgsl 没有 u8, 用 u32 凑活一下
alias Block = array<u32, 256>;

struct Lens {
    team_len: u32,
    work_count: u32,
    name_lens: array<u32>
}

/// 输入
// 统一的队名
@group(0) @binding(0) var<storage> team_name: U8BLock;
// 每个任务分别的队员名
@group(0) @binding(1) var<storage> names: array<U8BLock>;
// 各种长度
@group(0) @binding(2) var<storage> lens: Lens;

/// 输出
@group(0) @binding(3) var<storage, read_write> result: array<U8BLock>;

// 主函数
@compute @workgroup_size(16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // unpack4xU8
    let a = team_name[global_id.x % lens.team_len];
    let b = names[global_id.x / lens.team_len][global_id.x % lens.team_len];
    result[0][global_id.x] = a;
    // result[global_id.x][0] = a; // or any other appropriate value
}