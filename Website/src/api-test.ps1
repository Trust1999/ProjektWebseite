Invoke-RestMethod -Uri http://localhost:3000/api/autor{id} -Method Get

Invoke-RestMethod -Uri http://localhost:3000/api/autor -Method Post

$Params = @{
    Uri = 'http://localhost:3000/api/autor'
    Method = 'Post'
    Body = @{
        name = 'Bernd das Brot'
        land = 'Deutschland'
    } | ConvertTo-Json
    ContentType = 'application/json'
}
Invoke-RestMethod @Params

$Params = @{
    Uri = 'http://localhost:3000/api/autor/1'
    Method = 'Get'
}
Invoke-RestMethod @Params

psql -U postgres