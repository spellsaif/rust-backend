Invoke-WebRequest `
    -Uri "http://localhost:3001/users" `
    -Method POST `
    -Body '{"name": "nanasi", "email": "nanasi@isekai.com"}' `
    -ContentType "application/json"