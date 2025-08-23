Invoke-WebRequest `
    -Uri "http://localhost:3001/user" `
    -Method POST `
    -Body '{"name": "saif", "email": "saif@saif.com"}' `
    -ContentType "application/json"