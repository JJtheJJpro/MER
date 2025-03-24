export type x86_16_HiRegType = 'ah' | 'bh' | 'ch' | 'dh';
export type x86_16_LoRegType = 'al' | 'bl' | 'cl' | 'dl';
export type x86_16_HiLoRegType = 'ax' | 'bx' | 'cx' | 'dx';
export type x86_16_SegmRegType = 'cs' | 'ds' | 'ss' | 'es';
export type x86_16_PtrxRegType = 'si' | 'di' | 'bp' | 'sp';
export type x86_16_SpecRegType = 'ip' | 'flags';
export type x86_16_WordRegType = x86_16_HiLoRegType | x86_16_SegmRegType | x86_16_PtrxRegType | x86_16_SpecRegType;
export type x86_16_RegType = x86_16_HiRegType | x86_16_LoRegType | x86_16_WordRegType;