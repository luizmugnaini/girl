/// 16 KiB ROM bank 00. This bank stores the ROM data from cartridge that shall
/// be accessible throughout the whole game, usually being a fixed bank.
///
/// The following addresses are supposed to be used as jump vectors:
/// - RST instructions (1-byte): 0000, 0008, 0010, 0018, 0020, 0028, 0030, 0038.
/// - Interrupts: 0040, 0048, 0050, 0058, 0060.
const ROM_BANK_00: (usize, usize) = (0x0000, 0x3fff);

// TODO: what is NN here? is it a variable number so it can't be explicitly
// written? Could not find information about it for the time being.
/// 16 KiB ROM Bank 01 to NN. This bank stores the ROM data from cartridge and
/// is switchable via mapper.
const ROM_BANK_01_TO_NN: (usize, usize) = (0x4000, 0x7fff);

/// 8 KiB of video RAM.
const VRAM: (usize, usize) = (0x8000, 0x9fff);

/// 8 KiB of external RAM.
const EXTERNAL_RAM: (usize, usize) = (0xa000, 0xbfff);

/// 4 KiB of work RAM.
const WRAM1: (usize, usize) = (0xc000, 0xcfff);

/// 4 KiB of work RAM.
const WRAM2: (usize, usize) = (0xd000, 0xdfff);

/// Mirror of addresses `0xc000` to `0xddff`, that is, all reads and writes to
/// this range have the same effect as reads and writes to `0xc000` to `0xddff`.
/// This area has a forbidden access.
const PROHIBITED_ECHO_RAM: (usize, usize) = (0xe000, 0xfdff);

/// Sprite attribute table.
const SPRITE_OAM: (usize, usize) = (0xfe00, 0xfe9f);

/// Non-usable memory, its access is forbidden. This area returns `0xff` when
/// OAM is blocked, and otherwise the behavior depends on the hardware revision.
const PROHIBITED_MEM: (usize, usize) = (0xfea0, 0xfeff);

/// Input/output registers.
const IO_REGISTERS: (usize, usize) = (0xff00, 0xff7f);

/// High RAM.
const HRAM: (usize, usize) = (0xff80, 0xfffe);

/// Interrupt Enable register.
const IE_REGISTER: (usize, usize) = (0xffff, 0xffff);
