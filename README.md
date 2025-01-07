# yaml-cli

Adventure in managing yaml files into a merged data, focus in keeping yaml1.2 still working

## Base idea

rust based program 
command line that takes a collection of yaml files, that use yaml1.2, in a sequence
Merge their content, matching the values, and making the last file overrides previous value 

yw merge --input tests/dir1/input1.yaml --input /tests/dir1/input2.yaml --output /tests/output/output1.yaml
yw merge --input tests/dir1/ --output output.yaml

yw execute --input1 tests/output/output1.yaml --output tests/output/execute_output1.yaml

cargo run -- execute --input1 tests/output/output1.yaml --output tests/output/execute_output1.yaml

## Help me understand

At some point I need to represent my cloud as metadata (yaml), I will also need to use that metadata to execute commands.

## Azure prep

APIs ref: https://learn.microsoft.com/en-us/rest/api/azure/?view=rest-appservice-2024-04-01



## Ideas

### to consider
- Telemetry with App Insights
- Key Vault access for secrets
- App Registration in Entra ID
- Reporting azure with Kusto and Markdown

### Potentials ...
- Azure Audit rules engine and reporting
- https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file


## Author

[Marcio Parente](https://github.com/deixei) from deixei.com

To understand the overall context of this project read this book: [ENTERPRISE SOFTWARE DELIVERY: A ROADMAP FOR THE FUTURE](https://www.amazon.de/-/en/Marcio-Parente/dp/B0CXTJZJ2X/)