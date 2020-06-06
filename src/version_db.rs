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
            RsParams {
                bs: 0,
                dw: 0,
                ns: 0,
            },
            RsParams {
                bs: 0,
                dw: 0,
                ns: 0,
            },
            RsParams {
                bs: 0,
                dw: 0,
                ns: 0,
            },
            RsParams {
                bs: 0,
                dw: 0,
                ns: 0,
            },
        ],
    },
    VersionInfo {
        data_bytes: 26,
        apat: [0, 0, 0, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 26,
                dw: 16,
                ns: 1,
            },
            RsParams {
                bs: 26,
                dw: 19,
                ns: 1,
            },
            RsParams {
                bs: 26,
                dw: 9,
                ns: 1,
            },
            RsParams {
                bs: 26,
                dw: 13,
                ns: 1,
            },
        ],
    },
    {
        let init = VersionInfo {
            data_bytes: 44,
            apat: [6, 18, 0, 0, 0, 0, 0],
            ecc: [
                RsParams {
                    bs: 44,
                    dw: 28,
                    ns: 1,
                },
                RsParams {
                    bs: 44,
                    dw: 34,
                    ns: 1,
                },
                RsParams {
                    bs: 44,
                    dw: 16,
                    ns: 1,
                },
                RsParams {
                    bs: 44,
                    dw: 22,
                    ns: 1,
                },
            ],
        };
        init
    },
    {
        let init = VersionInfo {
            data_bytes: 70,
            apat: [6, 22, 0, 0, 0, 0, 0],
            ecc: [
                RsParams {
                    bs: 70,
                    dw: 44,
                    ns: 1,
                },
                RsParams {
                    bs: 70,
                    dw: 55,
                    ns: 1,
                },
                RsParams {
                    bs: 35,
                    dw: 13,
                    ns: 2,
                },
                RsParams {
                    bs: 35,
                    dw: 17,
                    ns: 2,
                },
            ],
        };
        init
    },
    {
        let init = VersionInfo {
            data_bytes: 100,
            apat: [6, 26, 0, 0, 0, 0, 0],
            ecc: [
                RsParams {
                    bs: 50,
                    dw: 32,
                    ns: 2,
                },
                RsParams {
                    bs: 100,
                    dw: 80,
                    ns: 1,
                },
                RsParams {
                    bs: 25,
                    dw: 9,
                    ns: 4,
                },
                RsParams {
                    bs: 50,
                    dw: 24,
                    ns: 2,
                },
            ],
        };
        init
    },
    VersionInfo {
        data_bytes: 134,
        apat: [6, 30, 0, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 67,
                dw: 43,
                ns: 2,
            },
            RsParams {
                bs: 134,
                dw: 108,
                ns: 1,
            },
            RsParams {
                bs: 33,
                dw: 11,
                ns: 2,
            },
            RsParams {
                bs: 33,
                dw: 15,
                ns: 2,
            },
        ],
    },
    VersionInfo {
        data_bytes: 172,
        apat: [6, 34, 0, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 43,
                dw: 27,
                ns: 4,
            },
            RsParams {
                bs: 86,
                dw: 68,
                ns: 2,
            },
            RsParams {
                bs: 43,
                dw: 15,
                ns: 4,
            },
            RsParams {
                bs: 43,
                dw: 19,
                ns: 4,
            },
        ],
    },
    VersionInfo {
        data_bytes: 196,
        apat: [6, 22, 38, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 49,
                dw: 31,
                ns: 4,
            },
            RsParams {
                bs: 98,
                dw: 78,
                ns: 2,
            },
            RsParams {
                bs: 39,
                dw: 13,
                ns: 4,
            },
            RsParams {
                bs: 32,
                dw: 14,
                ns: 2,
            },
        ],
    },
    VersionInfo {
        data_bytes: 242,
        apat: [6, 24, 42, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 60,
                dw: 38,
                ns: 2,
            },
            RsParams {
                bs: 121,
                dw: 97,
                ns: 2,
            },
            RsParams {
                bs: 40,
                dw: 14,
                ns: 4,
            },
            RsParams {
                bs: 40,
                dw: 18,
                ns: 4,
            },
        ],
    },
    VersionInfo {
        data_bytes: 292,
        apat: [6, 26, 46, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 58,
                dw: 36,
                ns: 3,
            },
            RsParams {
                bs: 146,
                dw: 116,
                ns: 2,
            },
            RsParams {
                bs: 36,
                dw: 12,
                ns: 4,
            },
            RsParams {
                bs: 36,
                dw: 16,
                ns: 4,
            },
        ],
    },
    VersionInfo {
        data_bytes: 346,
        apat: [6, 28, 50, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 69,
                dw: 43,
                ns: 4,
            },
            RsParams {
                bs: 86,
                dw: 68,
                ns: 2,
            },
            RsParams {
                bs: 43,
                dw: 15,
                ns: 6,
            },
            RsParams {
                bs: 43,
                dw: 19,
                ns: 6,
            },
        ],
    },
    VersionInfo {
        data_bytes: 404,
        apat: [6, 30, 54, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 80,
                dw: 50,
                ns: 1,
            },
            RsParams {
                bs: 101,
                dw: 81,
                ns: 4,
            },
            RsParams {
                bs: 36,
                dw: 12,
                ns: 3,
            },
            RsParams {
                bs: 50,
                dw: 22,
                ns: 4,
            },
        ],
    },
    VersionInfo {
        data_bytes: 466,
        apat: [6, 32, 58, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 58,
                dw: 36,
                ns: 6,
            },
            RsParams {
                bs: 116,
                dw: 92,
                ns: 2,
            },
            RsParams {
                bs: 42,
                dw: 14,
                ns: 7,
            },
            RsParams {
                bs: 46,
                dw: 20,
                ns: 4,
            },
        ],
    },
    VersionInfo {
        data_bytes: 532,
        apat: [6, 34, 62, 0, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 59,
                dw: 37,
                ns: 8,
            },
            RsParams {
                bs: 133,
                dw: 107,
                ns: 4,
            },
            RsParams {
                bs: 33,
                dw: 11,
                ns: 12,
            },
            RsParams {
                bs: 44,
                dw: 20,
                ns: 8,
            },
        ],
    },
    VersionInfo {
        data_bytes: 581,
        apat: [6, 26, 46, 66, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 64,
                dw: 40,
                ns: 4,
            },
            RsParams {
                bs: 145,
                dw: 115,
                ns: 3,
            },
            RsParams {
                bs: 36,
                dw: 12,
                ns: 11,
            },
            RsParams {
                bs: 36,
                dw: 16,
                ns: 11,
            },
        ],
    },
    VersionInfo {
        data_bytes: 655,
        apat: [6, 26, 48, 70, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 65,
                dw: 41,
                ns: 5,
            },
            RsParams {
                bs: 109,
                dw: 87,
                ns: 5,
            },
            RsParams {
                bs: 36,
                dw: 12,
                ns: 11,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 5,
            },
        ],
    },
    VersionInfo {
        data_bytes: 733,
        apat: [6, 26, 50, 74, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 73,
                dw: 45,
                ns: 7,
            },
            RsParams {
                bs: 122,
                dw: 98,
                ns: 5,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 3,
            },
            RsParams {
                bs: 43,
                dw: 19,
                ns: 15,
            },
        ],
    },
    VersionInfo {
        data_bytes: 815,
        apat: [6, 30, 54, 78, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 74,
                dw: 46,
                ns: 10,
            },
            RsParams {
                bs: 135,
                dw: 107,
                ns: 1,
            },
            RsParams {
                bs: 42,
                dw: 14,
                ns: 2,
            },
            RsParams {
                bs: 50,
                dw: 22,
                ns: 1,
            },
        ],
    },
    VersionInfo {
        data_bytes: 901,
        apat: [6, 30, 56, 82, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 69,
                dw: 43,
                ns: 9,
            },
            RsParams {
                bs: 150,
                dw: 120,
                ns: 5,
            },
            RsParams {
                bs: 42,
                dw: 14,
                ns: 2,
            },
            RsParams {
                bs: 50,
                dw: 22,
                ns: 17,
            },
        ],
    },
    VersionInfo {
        data_bytes: 991,
        apat: [6, 30, 58, 86, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 70,
                dw: 44,
                ns: 3,
            },
            RsParams {
                bs: 141,
                dw: 113,
                ns: 3,
            },
            RsParams {
                bs: 39,
                dw: 13,
                ns: 9,
            },
            RsParams {
                bs: 47,
                dw: 21,
                ns: 17,
            },
        ],
    },
    VersionInfo {
        data_bytes: 1085,
        apat: [6, 34, 62, 90, 0, 0, 0],
        ecc: [
            RsParams {
                bs: 67,
                dw: 41,
                ns: 3,
            },
            RsParams {
                bs: 135,
                dw: 107,
                ns: 3,
            },
            RsParams {
                bs: 43,
                dw: 15,
                ns: 15,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 15,
            },
        ],
    },
    VersionInfo {
        data_bytes: 1156,
        apat: [6, 28, 50, 72, 92, 0, 0],
        ecc: [
            RsParams {
                bs: 68,
                dw: 42,
                ns: 17,
            },
            RsParams {
                bs: 144,
                dw: 116,
                ns: 4,
            },
            RsParams {
                bs: 46,
                dw: 16,
                ns: 19,
            },
            RsParams {
                bs: 50,
                dw: 22,
                ns: 17,
            },
        ],
    },
    VersionInfo {
        data_bytes: 1258,
        apat: [6, 26, 50, 74, 98, 0, 0],
        ecc: [
            RsParams {
                bs: 74,
                dw: 46,
                ns: 17,
            },
            RsParams {
                bs: 139,
                dw: 111,
                ns: 2,
            },
            RsParams {
                bs: 37,
                dw: 13,
                ns: 34,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 7,
            },
        ],
    },
    VersionInfo {
        data_bytes: 1364,
        apat: [6, 30, 54, 78, 102, 0, 0],
        ecc: [
            RsParams {
                bs: 75,
                dw: 47,
                ns: 4,
            },
            RsParams {
                bs: 151,
                dw: 121,
                ns: 4,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 16,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 11,
            },
        ],
    },
    VersionInfo {
        data_bytes: 1474,
        apat: [6, 28, 54, 80, 106, 0, 0],
        ecc: [
            RsParams {
                bs: 73,
                dw: 45,
                ns: 6,
            },
            RsParams {
                bs: 147,
                dw: 117,
                ns: 6,
            },
            RsParams {
                bs: 46,
                dw: 16,
                ns: 30,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 11,
            },
        ],
    },
    VersionInfo {
        data_bytes: 1588,
        apat: [6, 32, 58, 84, 110, 0, 0],
        ecc: [
            RsParams {
                bs: 75,
                dw: 47,
                ns: 8,
            },
            RsParams {
                bs: 132,
                dw: 106,
                ns: 8,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 22,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 7,
            },
        ],
    },
    VersionInfo {
        data_bytes: 1706,
        apat: [6, 30, 58, 86, 114, 0, 0],
        ecc: [
            RsParams {
                bs: 74,
                dw: 46,
                ns: 19,
            },
            RsParams {
                bs: 142,
                dw: 114,
                ns: 10,
            },
            RsParams {
                bs: 46,
                dw: 16,
                ns: 33,
            },
            RsParams {
                bs: 50,
                dw: 22,
                ns: 28,
            },
        ],
    },
    VersionInfo {
        data_bytes: 1828,
        apat: [6, 34, 62, 90, 118, 0, 0],
        ecc: [
            RsParams {
                bs: 73,
                dw: 45,
                ns: 22,
            },
            RsParams {
                bs: 152,
                dw: 122,
                ns: 8,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 12,
            },
            RsParams {
                bs: 53,
                dw: 23,
                ns: 8,
            },
        ],
    },
    VersionInfo {
        data_bytes: 1921,
        apat: [6, 26, 50, 74, 98, 122, 0],
        ecc: [
            RsParams {
                bs: 73,
                dw: 45,
                ns: 3,
            },
            RsParams {
                bs: 147,
                dw: 117,
                ns: 3,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 11,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 4,
            },
        ],
    },
    VersionInfo {
        data_bytes: 2051,
        apat: [6, 30, 54, 78, 102, 126, 0],
        ecc: [
            RsParams {
                bs: 73,
                dw: 45,
                ns: 21,
            },
            RsParams {
                bs: 146,
                dw: 116,
                ns: 7,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 19,
            },
            RsParams {
                bs: 53,
                dw: 23,
                ns: 1,
            },
        ],
    },
    VersionInfo {
        data_bytes: 2185,
        apat: [6, 26, 52, 78, 104, 130, 0],
        ecc: [
            RsParams {
                bs: 75,
                dw: 47,
                ns: 19,
            },
            RsParams {
                bs: 145,
                dw: 115,
                ns: 5,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 23,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 15,
            },
        ],
    },
    VersionInfo {
        data_bytes: 2323,
        apat: [6, 30, 56, 82, 108, 134, 0],
        ecc: [
            RsParams {
                bs: 74,
                dw: 46,
                ns: 2,
            },
            RsParams {
                bs: 145,
                dw: 115,
                ns: 13,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 23,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 42,
            },
        ],
    },
    VersionInfo {
        data_bytes: 2465,
        apat: [6, 34, 60, 86, 112, 138, 0],
        ecc: [
            RsParams {
                bs: 74,
                dw: 46,
                ns: 10,
            },
            RsParams {
                bs: 145,
                dw: 115,
                ns: 17,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 19,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 10,
            },
        ],
    },
    VersionInfo {
        data_bytes: 2611,
        apat: [6, 30, 58, 86, 114, 142, 0],
        ecc: [
            RsParams {
                bs: 74,
                dw: 46,
                ns: 14,
            },
            RsParams {
                bs: 145,
                dw: 115,
                ns: 17,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 11,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 29,
            },
        ],
    },
    VersionInfo {
        data_bytes: 2761,
        apat: [6, 34, 62, 90, 118, 146, 0],
        ecc: [
            RsParams {
                bs: 74,
                dw: 46,
                ns: 14,
            },
            RsParams {
                bs: 145,
                dw: 115,
                ns: 13,
            },
            RsParams {
                bs: 46,
                dw: 16,
                ns: 59,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 44,
            },
        ],
    },
    VersionInfo {
        data_bytes: 2876,
        apat: [6, 30, 54, 78, 102, 126, 150],
        ecc: [
            RsParams {
                bs: 75,
                dw: 47,
                ns: 12,
            },
            RsParams {
                bs: 151,
                dw: 121,
                ns: 12,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 22,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 39,
            },
        ],
    },
    VersionInfo {
        data_bytes: 3034,
        apat: [6, 24, 50, 76, 102, 128, 154],
        ecc: [
            RsParams {
                bs: 75,
                dw: 47,
                ns: 6,
            },
            RsParams {
                bs: 151,
                dw: 121,
                ns: 6,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 2,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 46,
            },
        ],
    },
    VersionInfo {
        data_bytes: 3196,
        apat: [6, 28, 54, 80, 106, 132, 158],
        ecc: [
            RsParams {
                bs: 74,
                dw: 46,
                ns: 29,
            },
            RsParams {
                bs: 152,
                dw: 122,
                ns: 17,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 24,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 49,
            },
        ],
    },
    VersionInfo {
        data_bytes: 3362,
        apat: [6, 32, 58, 84, 110, 136, 162],
        ecc: [
            RsParams {
                bs: 74,
                dw: 46,
                ns: 13,
            },
            RsParams {
                bs: 152,
                dw: 122,
                ns: 4,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 42,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 48,
            },
        ],
    },
    VersionInfo {
        data_bytes: 3532,
        apat: [6, 26, 54, 82, 110, 138, 166],
        ecc: [
            RsParams {
                bs: 75,
                dw: 47,
                ns: 40,
            },
            RsParams {
                bs: 147,
                dw: 117,
                ns: 20,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 10,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 43,
            },
        ],
    },
    VersionInfo {
        data_bytes: 3706,
        apat: [6, 30, 58, 86, 114, 142, 170],
        ecc: [
            RsParams {
                bs: 75,
                dw: 47,
                ns: 18,
            },
            RsParams {
                bs: 148,
                dw: 118,
                ns: 19,
            },
            RsParams {
                bs: 45,
                dw: 15,
                ns: 20,
            },
            RsParams {
                bs: 54,
                dw: 24,
                ns: 34,
            },
        ],
    },
];
