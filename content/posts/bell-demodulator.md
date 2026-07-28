---
title: "A Bell 103 Demodulator"
kind: project
website: https://github.com/lukexor/bell103_demodulator
published: 2019-04-26T02:42:15Z
started: 2019-04-26T02:42:01Z
completed: 2019-04-26T02:42:04Z
updated: 2020-01-23T02:42:18Z
---

This was done as a class project and is the basic start to implementing a full
Bell 103 modem. Currently, only the demodulating portion is implemented and is
not fully featured.

It is currently capable of decoding 48,000 kilosample/s 16-bit single-channel WAV
files in little-endian Microsoft PCM format. The file contents must be encoded
using the answering frequencies of the 9N1 Bell 103 protocol at 300 bits per
second. The bytes must also be packed tight with no lead-in or filtering.

There are options for changing the sampling rate, and filter length as well as
using the origin frequencies instead of answering but these have not been
tested.
