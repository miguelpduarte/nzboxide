# NZBOxide

NZBOxide aims to be NZB news reader with feature parity with [NZBGet](https://github.com/nzbget/nzbget), providing a modern alternative with a similar level of efficiency.

Keep in mind that this software is in **very early stages** and is just a small hobby project, so use it at your own risk.

## Goals

- Basic feature parity with NZBGet
- Speed
- Efficiency in constrained devices (SABnzbd has been reported to be a bit too memory-intensive for some smaller devices)
- Interoperability with *arr suite

## Wishlist

- [x] Parsing of NZB file
- [ ] Download file parts
    - [ ] Speak NNTP (might need to reimplement this as crates related to it either don't handle the binary part, or are apparently unmaintained)
- [ ] Merge file parts
- [ ] CRC checks after file downloads
- [ ] Pre-flight checks
- [ ] Integration testing with [NServ](https://nzbget.net/nserv-nntp-server)
