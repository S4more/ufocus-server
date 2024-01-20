# Software engineer and flowers
POST http://localhost:3000
{
  "page_body": "this is a website that talk about flowers",
  "user_agent": "I'm a software engineer trying to implement AWS in terraform"
}
HTTP 200
[Asserts]
jsonpath "$.relevance" < 6
jsonpath "$.keywords" count > 0

# Software engineer and blockchains
POST http://localhost:3000
{
  "page_body": "How you can use rust to write a blockchain",
  "user_agent": "I'm a software engineer trying to implement AWS in terraform"
}
HTTP 200
[Asserts]
jsonpath "$.relevance" > 6 # Should this be over 6?
jsonpath "$.keywords" count > 0
jsonpath "$.reason" != null
jsonpath "$.reason" != null