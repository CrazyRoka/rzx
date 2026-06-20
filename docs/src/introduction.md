# Introduction

Welcome to the ZX Spectrum Hardware Documentation. The documentation is scattered across decades of newsgroup posts, individual emulator source code, hardware manuals, and fragmented wikis. This book is an attempt to consolidate that information into a single, locally hosted, emulator-focused reference.

### Why the Spectrum?

The Sinclair ZX Spectrum is a fascinating machine to emulate. Released in 1982, the Spectrum relies on absolute minimalism. 

There is no graphics chip. There is no sound chip. There is barely an I/O controller. The machine is essentially a Z80 CPU, a block of RAM, and a custom gate array called the ULA that sneaks reads from the RAM behind the CPU's back to generate a video signal. This minimalist design leads to a machine that is incredibly simple in theory, but rife with quirks, timing dependencies, and "clever" software hacks in practice.

### Scope of this Book

This book focuses primarily on the **ZX Spectrum**.