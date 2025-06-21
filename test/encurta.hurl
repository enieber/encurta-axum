POST http://localhost:3000/api/v1/router
{
    "link": "https://mapeamento.softwarelivre.tec.br/"
}
HTTP 200
[Asserts]
jsonpath "$.link" == "https://mapeamento.softwarelivre.tec.br/"
jsonpath "$.hash" == "6746943"


GET http://localhost:3000/api/v1/router/6746943
HTTP 200
[Asserts]
jsonpath "$.link" == "https://mapeamento.softwarelivre.tec.br/"
jsonpath "$.hash" == "6746943"
