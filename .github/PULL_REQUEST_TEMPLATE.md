---
name: Pull Request
about: Generic template to create pull requests to this repo
title: ''
labels: ''
assignees: ''
projects: NodleCode/3

---

## Context
Describe what is changed and your reasoning.

> Fixes [if needed, mention the issues involved].

### Sanity
- [ ] I have incremented the runtime version number
- [ ] I have incremented `transaction_version` if needed

### Quality
- [ ] I have added unit tests, and they are passing
- [ ] I have added benchmarks to my pallet
- [ ] I have added the benchmarks to the node
- [ ] I have added potential RPC calls to the node
- [ ] I have added comments and documentation

### Testing
- [ ] The node runs fine on a development network
- [ ] The node runs fine on a local network
- [ ] The node runs fine on an upgraded local network
- [ ] The node can synchronize the existing network