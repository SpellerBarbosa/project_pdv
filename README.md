# PDV Mercadinho 🛒

[![Status](https://img.shields.io/badge/status-em%20desenvolvimento-yellow)](https://github.com/SpellerBarbosa/project_pdv)  
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

---

## Sobre o projeto

**PDV Mercadinho** é um sistema de ponto de venda desenvolvido para mercadinhos. Este projeto foi criado para **fins de estudo**, para **demonstrar minhas habilidades** e **compor meu portfólio**.

O sistema roda **localmente** utilizando **Actix-web (Rust)** para gerenciar a aplicação, criar o banco de dados e fazer a conexão entre o frontend e o banco. Futuramente, poderá ser escalado para uma API web.

A escolha pelo funcionamento local garante:

- Testes seguros em cada máquina
- Nenhuma exposição de dados na rede
- Custos zero, pois todos os dados são armazenados localmente

---

## Tecnologias utilizadas

| Camada       | Tecnologia                 |
|--------------|----------------------------|
| Frontend     | Nuxt4, TailwindCSS         |
| Backend      | Actix-web (Rust), SQLite   |
| Empacotador  | Tauri                      |

---

## Funcionalidades (em desenvolvimento)

- Tela de **login** utilizando Nuxt4 e TailwindCSS
- API com Actix-web contendo duas rotas:
  - `/` → Retorna "Servidor funcionando"
  - `/register-user` → Para registrar usuários

**Observação:** O frontend ainda não está conectado ao backend.

---
### 🗂 Estrutura do projeto
``` bash
project_pdv/
├─ app/                  # Código-fonte do frontend
├─ public/               # Arquivos públicos
├─ src-tauri/            # Código do backend Actix-web
├─ .gitignore
├─ README.md
├─ nuxt.config.ts
├─ package-lock.json
├─ package.json
├─ tsconfig.json
└─ LICENSE

 ```
 ---

### 📦 Banco de dados (SQLite)

Arquivo: pdv_database.sqlite

Tabela users:
| Campo | Tipo | Observação |
|------------|-----------|------------|
| id | INTEGER | PK, AUTOINCREMENT |
| name | TEXT | Nome do usuário |
| user | TEXT | Login |
| password | TEXT | Senha |
| role | TEXT | Papel do usuário |
| create_at | DATETIME | Criado automaticamente com timestamp |

---

### 🛠 Próximos passos (Backlog)

Criar endpoints CRUD para users

Criar tabela products e endpoints de produtos

Implementar autenticação local

Conectar frontend Tauri com backend JSON

---

## Como rodar localmente

### Requisitos

1. **Tauri**: siga os passos de instalação neste link: [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites)
   - Inclui Rust e Cargo, necessários para o funcionamento da aplicação.
2. **Node.js** versão 19+ para compatibilidade com Nuxt4 e TailwindCSS

### Passos

```bash
# Faça o clone do repositório
git clone https://github.com/SpellerBarbosa/project_pdv

# Entre na pasta do projeto
cd project_pdv

# Instale as dependências
npm install

# Rode a aplicação
npx tauri dev
