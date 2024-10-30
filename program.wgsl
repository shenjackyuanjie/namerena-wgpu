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
// 已经转换好的 u32 数组, 减少解包的次数, 提高一点点效率
@group(0) @binding(0) var<storage> team_name: Block;
// 每个任务分别的队员名
@group(0) @binding(1) var<storage> names: array<U8BLock>;
// 各种长度
@group(0) @binding(2) var<storage> lens: Lens;

/// 输出
@group(0) @binding(3) var<storage, read_write> result: array<U8BLock>;

// 主函数
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // unpack4xU8
    let a = team_name[global_id.x % lens.team_len];
    let b = names[global_id.x / lens.team_len][global_id.x % lens.team_len];
    
    let id = global_id.x;
    if id > lens.work_count + 10 {
        return;
    }
    var local_result: Block = array<u32, 256>(
    0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   10,  11,  12,  13,  14,  15,
    16,  17,  18,  19,  20,  21,  22,  23,  24,  25,  26,  27,  28,  29,  30,  31,
    32,  33,  34,  35,  36,  37,  38,  39,  40,  41,  42,  43,  44,  45,  46,  47,
    48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  58,  59,  60,  61,  62,  63,
    64,  65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,
    80,  81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  91,  92,  93,  94,  95,
    96,  97,  98,  99,  100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,);

    var s = 0u;
    for (var i = 0u; i < 256u; i += 1u) {
        if ((i % lens.team_len) != 0) {
            s = (s + team_name[i % lens.team_len - 1u]) & 256;
        }
        s = (s + local_result[i]) % 256;
        var tmp = local_result[i];
        local_result[i] = local_result[s];
        local_result[s] = tmp;
    }

    
    // result[0][id] = team_name[id] + 10;
    // result[global_id.x][0] = a; // or any other appropriate value
    // 把数据塞回去
    for (var i = 0u; i < 256u; i = i + 4u) {
        let v = vec4<u32>(
            local_result[i],
            local_result[i + 1u],
            local_result[i + 2u],
            local_result[i + 3u]
        );
        let packed: u32 = pack4xU8(v);
        result[id][i / 4u] = packed;
    }
}