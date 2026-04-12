$Password = Read-Host -AsSecureString "Enter a test password"
New-LocalUser "DenTest" -Password $Password -Description "Standard test account"