param([Parameter(Mandatory=$true)][string]$SourceIdentity,
      [Parameter(Mandatory=$true)][switch]$SecondHumanReviewRecorded)
$ErrorActionPreference = 'Stop'
if (-not $SecondHumanReviewRecorded) { throw 'Deployment is blocked pending second human custody review.' }
Write-Host 'Testnet only. This script does not authorize real funds or claim a pilot.'
stellar contract build
if ($LASTEXITCODE -ne 0) { throw 'Contract build failed.' }
stellar contract deploy --wasm target/wasm32v1-none/release/ajo.wasm --source $SourceIdentity --network testnet
if ($LASTEXITCODE -ne 0) { throw 'Deployment failed.' }
