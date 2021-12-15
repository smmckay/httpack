# Overall Structure

1. Header page `x 1`
2. Lookup page `x lookup_page_count`
3. File data `x data_file_count`

# Header Page

```
┏━━━━━━┳━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ Type ┃ Name              ┃ Comment                                           ┃
┣━━━━━━╋━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃  u64 ┃             magic ┃ Constant 0x13243546ACBDCEDF                       ┃
┃  u64 ┃           version ┃ Constant 0x0000000000000001                       ┃
┃  u64 ┃   data_file_count ┃ Number of files contained in this pack            ┃
┃  u64 ┃ lookup_page_count ┃                                                   ┃
┃    - ┃           padding ┃ NUL x 4064                                        ┃
┗━━━━━━┻━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

The header page is followed by `lookup_page_count` lookup pages.

# Lookup Page

```
┏━━━━━━━┳━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ Type  ┃ Name      ┃ Comment                                                  ┃
┣━━━━━━━╋━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃   u16 ┃ lpe_count ┃ Number of entries in this page                           ┃
┃ LPE[] ┃   entries ┃                                                          ┃
┃     - ┃   padding ┃ NUL to end of page                                       ┃
┗━━━━━━━┻━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

Each lookup page contains `lpe_count` entries.

# Lookup Page Entry (LPE)

```
┏━━━━━━┳━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ Type ┃ Name          ┃ Comment                                               ┃
┣━━━━━━╋━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃  u16 ┃    prefix_len ┃ count of key bytes to reuse from previous LPE         ┃
┃  u16 ┃      tail_len ┃ count of key bytes to append to the prefix            ┃
┃  str ┃          tail ┃                                                       ┃
┃    - ┃       padding ┃ NUL to eight-bye boundary                             ┃
┃  u64 ┃    data_start ┃ offset in the pack where this entry's data begins     ┃
┃  u64 ┃  clear_length ┃ length of uncompressed (clear) data                   ┃
┃  u64 ┃   gzip_length ┃ length of gzipped data                                ┃
┃  u64 ┃ brotli_length ┃ length of brotli'd data                               ┃
┗━━━━━━┻━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

A lookup page entry describes the name of a file and its location within the
pack. The first entry in a lookup page will always have `prefix_len == 0`,
because there is no previous entry to refer to. As a result, the tail of the 
first entry will have the entire first key. Each entry following the first uses
`prefix_len` to reference that many bytes from the beginning of the _previous_
key, not the first. Thus `prefix_len` in the third entry refers to the
reconstructed second key, `prefix_len` in the fourth entry refers to the
reconstructed third key, and so on.

# File data

```
┏━━━━━━┳━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ Type ┃ Name        ┃ Comment                                                 ┃
┣━━━━━━╋━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃  bin ┃  clear_data ┃ uncompressed file data                                  ┃
┃  bin ┃   gzip_data ┃ gzipped file data                                       ┃
┃  bin ┃ brotli_data ┃ brotli'd file data                                      ┃
┗━━━━━━┻━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
