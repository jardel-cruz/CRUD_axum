# CRUD_axum

Esse projeto escrito em RUST trás um crud simples utilizando o framework [Axum](https://github.com/tokio-rs/axum) como base,
o projeto usa um banco de dados simples implementado manualmente que armazena
os dados em memória, o [Tokio](https://github.com/tokio-rs/tokio) como runtime assíncrono e o
[Serde](https://github.com/serde-rs/serde) para o parser das requisições http.

O servidor é responsável por registrar, buscar, atualizar e excluir usuários, também será implementado um sistema de login
por email e senha.
