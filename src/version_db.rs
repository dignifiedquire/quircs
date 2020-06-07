pub const VERSION_MIN: usize = 1;
pub const VERSION_MAX: usize = 40;

/// QR-code version information database
#[derive(Debug, Clone, Default)]
pub struct RsParams {
    /// Small block size.
    pub bs: i32,
    /// Small data words.
    pub dw: i32,
    /// Number of small blocks.
    pub ns: i32,
}

impl RsParams {
    pub const fn new(bs: i32, dw: i32, ns: i32) -> Self {
        Self { bs, dw, ns }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VersionInfo {
    pub data_bytes: i32,
    pub apat: [i32; 7],
    pub ecc: [RsParams; 4],
}

pub static VERSION_DB: [VersionInfo; 41] = [
    VersionInfo {
        data_bytes: 0,
        apat: [0; 7],
        ecc: [
            RsParams::new(0, 0, 0),
            RsParams::new(0, 0, 0),
            RsParams::new(0, 0, 0),
            RsParams::new(0, 0, 0),
        ],
    },
    VersionInfo {
        data_bytes: 26,
        apat: [0, 0, 0, 0, 0, 0, 0],
        ecc: [
            RsParams::new(26, 16, 1),
            RsParams::new(26, 19, 1),
            RsParams::new(26, 9, 1),
            RsParams::new(26, 13, 1),
        ],
    },
    VersionInfo {
        data_bytes: 44,
        apat: [6, 18, 0, 0, 0, 0, 0],
        ecc: [
            RsParams::new(44, 28, 1),
            RsParams::new(44, 34, 1),
            RsParams::new(44, 16, 1),
            RsParams::new(44, 22, 1),
        ],
    },
    VersionInfo {
        data_bytes: 70,
        apat: [6, 22, 0, 0, 0, 0, 0],
        ecc: [
            RsParams::new(70, 44, 1),
            RsParams::new(70, 55, 1),
            RsParams::new(35, 13, 2),
            RsParams::new(35, 17, 2),
        ],
    },
    VersionInfo {
        data_bytes: 100,
        apat: [6, 26, 0, 0, 0, 0, 0],
        ecc: [
            RsParams::new(50, 32, 2),
            RsParams::new(100, 80, 1),
            RsParams::new(25, 9, 4),
            RsParams::new(50, 24, 2),
        ],
    },
    VersionInfo {
        data_bytes: 134,
        apat: [6, 30, 0, 0, 0, 0, 0],
        ecc: [
            RsParams::new(67, 43, 2),
            RsParams::new(134, 108, 1),
            RsParams::new(33, 11, 2),
            RsParams::new(33, 15, 2),
        ],
    },
    VersionInfo {
        data_bytes: 172,
        apat: [6, 34, 0, 0, 0, 0, 0],
        ecc: [
            RsParams::new(43, 27, 4),
            RsParams::new(86, 68, 2),
            RsParams::new(43, 15, 4),
            RsParams::new(43, 19, 4),
        ],
    },
    VersionInfo {
        data_bytes: 196,
        apat: [6, 22, 38, 0, 0, 0, 0],
        ecc: [
            RsParams::new(49, 31, 4),
            RsParams::new(98, 78, 2),
            RsParams::new(39, 13, 4),
            RsParams::new(32, 14, 2),
        ],
    },
    VersionInfo {
        data_bytes: 242,
        apat: [6, 24, 42, 0, 0, 0, 0],
        ecc: [
            RsParams::new(60, 38, 2),
            RsParams::new(121, 97, 2),
            RsParams::new(40, 14, 4),
            RsParams::new(40, 18, 4),
        ],
    },
    VersionInfo {
        data_bytes: 292,
        apat: [6, 26, 46, 0, 0, 0, 0],
        ecc: [
            RsParams::new(58, 36, 3),
            RsParams::new(146, 116, 2),
            RsParams::new(36, 12, 4),
            RsParams::new(36, 16, 4),
        ],
    },
    VersionInfo {
        data_bytes: 346,
        apat: [6, 28, 50, 0, 0, 0, 0],
        ecc: [
            RsParams::new(69, 43, 4),
            RsParams::new(86, 68, 2),
            RsParams::new(43, 15, 6),
            RsParams::new(43, 19, 6),
        ],
    },
    VersionInfo {
        data_bytes: 404,
        apat: [6, 30, 54, 0, 0, 0, 0],
        ecc: [
            RsParams::new(80, 50, 1),
            RsParams::new(101, 81, 4),
            RsParams::new(36, 12, 3),
            RsParams::new(50, 22, 4),
        ],
    },
    VersionInfo {
        data_bytes: 466,
        apat: [6, 32, 58, 0, 0, 0, 0],
        ecc: [
            RsParams::new(58, 36, 6),
            RsParams::new(116, 92, 2),
            RsParams::new(42, 14, 7),
            RsParams::new(46, 20, 4),
        ],
    },
    VersionInfo {
        data_bytes: 532,
        apat: [6, 34, 62, 0, 0, 0, 0],
        ecc: [
            RsParams::new(59, 37, 8),
            RsParams::new(133, 107, 4),
            RsParams::new(33, 11, 12),
            RsParams::new(44, 20, 8),
        ],
    },
    VersionInfo {
        data_bytes: 581,
        apat: [6, 26, 46, 66, 0, 0, 0],
        ecc: [
            RsParams::new(64, 40, 4),
            RsParams::new(145, 115, 3),
            RsParams::new(36, 12, 11),
            RsParams::new(36, 16, 11),
        ],
    },
    VersionInfo {
        data_bytes: 655,
        apat: [6, 26, 48, 70, 0, 0, 0],
        ecc: [
            RsParams::new(65, 41, 5),
            RsParams::new(109, 87, 5),
            RsParams::new(36, 12, 11),
            RsParams::new(54, 24, 5),
        ],
    },
    VersionInfo {
        data_bytes: 733,
        apat: [6, 26, 50, 74, 0, 0, 0],
        ecc: [
            RsParams::new(73, 45, 7),
            RsParams::new(122, 98, 5),
            RsParams::new(45, 15, 3),
            RsParams::new(43, 19, 15),
        ],
    },
    VersionInfo {
        data_bytes: 815,
        apat: [6, 30, 54, 78, 0, 0, 0],
        ecc: [
            RsParams::new(74, 46, 10),
            RsParams::new(135, 107, 1),
            RsParams::new(42, 14, 2),
            RsParams::new(50, 22, 1),
        ],
    },
    VersionInfo {
        data_bytes: 901,
        apat: [6, 30, 56, 82, 0, 0, 0],
        ecc: [
            RsParams::new(69, 43, 9),
            RsParams::new(150, 120, 5),
            RsParams::new(42, 14, 2),
            RsParams::new(50, 22, 17),
        ],
    },
    VersionInfo {
        data_bytes: 991,
        apat: [6, 30, 58, 86, 0, 0, 0],
        ecc: [
            RsParams::new(70, 44, 3),
            RsParams::new(141, 113, 3),
            RsParams::new(39, 13, 9),
            RsParams::new(47, 21, 17),
        ],
    },
    VersionInfo {
        data_bytes: 1085,
        apat: [6, 34, 62, 90, 0, 0, 0],
        ecc: [
            RsParams::new(67, 41, 3),
            RsParams::new(135, 107, 3),
            RsParams::new(43, 15, 15),
            RsParams::new(54, 24, 15),
        ],
    },
    VersionInfo {
        data_bytes: 1156,
        apat: [6, 28, 50, 72, 92, 0, 0],
        ecc: [
            RsParams::new(68, 42, 17),
            RsParams::new(144, 116, 4),
            RsParams::new(46, 16, 19),
            RsParams::new(50, 22, 17),
        ],
    },
    VersionInfo {
        data_bytes: 1258,
        apat: [6, 26, 50, 74, 98, 0, 0],
        ecc: [
            RsParams::new(74, 46, 17),
            RsParams::new(139, 111, 2),
            RsParams::new(37, 13, 34),
            RsParams::new(54, 24, 7),
        ],
    },
    VersionInfo {
        data_bytes: 1364,
        apat: [6, 30, 54, 78, 102, 0, 0],
        ecc: [
            RsParams::new(75, 47, 4),
            RsParams::new(151, 121, 4),
            RsParams::new(45, 15, 16),
            RsParams::new(54, 24, 11),
        ],
    },
    VersionInfo {
        data_bytes: 1474,
        apat: [6, 28, 54, 80, 106, 0, 0],
        ecc: [
            RsParams::new(73, 45, 6),
            RsParams::new(147, 117, 6),
            RsParams::new(46, 16, 30),
            RsParams::new(54, 24, 11),
        ],
    },
    VersionInfo {
        data_bytes: 1588,
        apat: [6, 32, 58, 84, 110, 0, 0],
        ecc: [
            RsParams::new(75, 47, 8),
            RsParams::new(132, 106, 8),
            RsParams::new(45, 15, 22),
            RsParams::new(54, 24, 7),
        ],
    },
    VersionInfo {
        data_bytes: 1706,
        apat: [6, 30, 58, 86, 114, 0, 0],
        ecc: [
            RsParams::new(74, 46, 19),
            RsParams::new(142, 114, 10),
            RsParams::new(46, 16, 33),
            RsParams::new(50, 22, 28),
        ],
    },
    VersionInfo {
        data_bytes: 1828,
        apat: [6, 34, 62, 90, 118, 0, 0],
        ecc: [
            RsParams::new(73, 45, 22),
            RsParams::new(152, 122, 8),
            RsParams::new(45, 15, 12),
            RsParams::new(53, 23, 8),
        ],
    },
    VersionInfo {
        data_bytes: 1921,
        apat: [6, 26, 50, 74, 98, 122, 0],
        ecc: [
            RsParams::new(73, 45, 3),
            RsParams::new(147, 117, 3),
            RsParams::new(45, 15, 11),
            RsParams::new(54, 24, 4),
        ],
    },
    VersionInfo {
        data_bytes: 2051,
        apat: [6, 30, 54, 78, 102, 126, 0],
        ecc: [
            RsParams::new(73, 45, 21),
            RsParams::new(146, 116, 7),
            RsParams::new(45, 15, 19),
            RsParams::new(53, 23, 1),
        ],
    },
    VersionInfo {
        data_bytes: 2185,
        apat: [6, 26, 52, 78, 104, 130, 0],
        ecc: [
            RsParams::new(75, 47, 19),
            RsParams::new(145, 115, 5),
            RsParams::new(45, 15, 23),
            RsParams::new(54, 24, 15),
        ],
    },
    VersionInfo {
        data_bytes: 2323,
        apat: [6, 30, 56, 82, 108, 134, 0],
        ecc: [
            RsParams::new(74, 46, 2),
            RsParams::new(145, 115, 13),
            RsParams::new(45, 15, 23),
            RsParams::new(54, 24, 42),
        ],
    },
    VersionInfo {
        data_bytes: 2465,
        apat: [6, 34, 60, 86, 112, 138, 0],
        ecc: [
            RsParams::new(74, 46, 10),
            RsParams::new(145, 115, 17),
            RsParams::new(45, 15, 19),
            RsParams::new(54, 24, 10),
        ],
    },
    VersionInfo {
        data_bytes: 2611,
        apat: [6, 30, 58, 86, 114, 142, 0],
        ecc: [
            RsParams::new(74, 46, 14),
            RsParams::new(145, 115, 17),
            RsParams::new(45, 15, 11),
            RsParams::new(54, 24, 29),
        ],
    },
    VersionInfo {
        data_bytes: 2761,
        apat: [6, 34, 62, 90, 118, 146, 0],
        ecc: [
            RsParams::new(74, 46, 14),
            RsParams::new(145, 115, 13),
            RsParams::new(46, 16, 59),
            RsParams::new(54, 24, 44),
        ],
    },
    VersionInfo {
        data_bytes: 2876,
        apat: [6, 30, 54, 78, 102, 126, 150],
        ecc: [
            RsParams::new(75, 47, 12),
            RsParams::new(151, 121, 12),
            RsParams::new(45, 15, 22),
            RsParams::new(54, 24, 39),
        ],
    },
    VersionInfo {
        data_bytes: 3034,
        apat: [6, 24, 50, 76, 102, 128, 154],
        ecc: [
            RsParams::new(75, 47, 6),
            RsParams::new(151, 121, 6),
            RsParams::new(45, 15, 2),
            RsParams::new(54, 24, 46),
        ],
    },
    VersionInfo {
        data_bytes: 3196,
        apat: [6, 28, 54, 80, 106, 132, 158],
        ecc: [
            RsParams::new(74, 46, 29),
            RsParams::new(152, 122, 17),
            RsParams::new(45, 15, 24),
            RsParams::new(54, 24, 49),
        ],
    },
    VersionInfo {
        data_bytes: 3362,
        apat: [6, 32, 58, 84, 110, 136, 162],
        ecc: [
            RsParams::new(74, 46, 13),
            RsParams::new(152, 122, 4),
            RsParams::new(45, 15, 42),
            RsParams::new(54, 24, 48),
        ],
    },
    VersionInfo {
        data_bytes: 3532,
        apat: [6, 26, 54, 82, 110, 138, 166],
        ecc: [
            RsParams::new(75, 47, 40),
            RsParams::new(147, 117, 20),
            RsParams::new(45, 15, 10),
            RsParams::new(54, 24, 43),
        ],
    },
    VersionInfo {
        data_bytes: 3706,
        apat: [6, 30, 58, 86, 114, 142, 170],
        ecc: [
            RsParams::new(75, 47, 18),
            RsParams::new(148, 118, 19),
            RsParams::new(45, 15, 20),
            RsParams::new(54, 24, 34),
        ],
    },
];
