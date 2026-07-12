Invoke-RestMethod -Uri http://localhost:3000/api/autor{id} -Method Get

Invoke-RestMethod -Uri http://localhost:3000/api/autor -Method Post

$Params = @{
    Uri = 'http://localhost:3000/author'
    Method = 'Post'
    Body = @{
        name = 'Der Busch'
    } | ConvertTo-Json
    ContentType = 'application/json'
}
Invoke-RestMethod @Params

$Params = @{
    Uri = 'http://localhost:3000/author/c8679e36-addf-41c9-aac5-29c80ce8c3fa'
    Method = 'Get'
}
Invoke-RestMethod @Params

$Params = @{
    Uri = 'http://localhost:3000/author'
    Method = 'Get'
}
Invoke-RestMethod @Params

$Params = @{
    Uri = 'http://localhost:3000/author/35eb54d8-0ae4-47a6-9d53-926978e86cd3'
    Method = 'Delete'
}
Invoke-RestMethod @Params

$Params = @{
    Uri = 'http://localhost:3000/author/c8679e36-addf-41c9-aac5-29c80ce8c3fa'
    Method = 'Patch'
    Body = @{
        name = 'Der Baum'
    } | ConvertTo-Json
    ContentType = 'application/json'
}
Invoke-RestMethod @Params

psql -U postgres