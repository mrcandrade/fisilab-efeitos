# Modificações do Efeitos em relação ao EffectCraft

O Efeitos é uma versão modificada do [EffectCraft](https://github.com/storytold/effectcraft)
(© 2026 ArtCraft Team e contribuidores do EffectCraft, MIT OR Apache-2.0), disponibilizada pelo
**FisiLab — LARFIC · UFRGS** em parceria com o **CTA — Centro de Tecnologia Acadêmica** como parte
da **Suíte FisiLab**, para democratizar o acesso à tecnologia entre os alunos da UFRGS. As modificações são
© 2026 FisiLab — LARFIC/UFRGS e seguem os mesmos termos (MIT OR Apache-2.0).

Base: EffectCraft 0.10.0 (commit `05955c0`, "Release 0.10.0").

## O que foi alterado, e por quê

### Nome e marca

- **Nome visível ao usuário: EffectCraft → Efeitos** — título da janela, nome do app no
  `eframe`, menus ("Sobre o Efeitos…", "Sair do Efeitos"…), diálogos, tela inicial, dicas,
  mensagens de erro e de status, saída de `--version`/ajuda da linha de comando, e os catálogos de
  tradução (chaves e valores atualizados juntos, em todos os idiomas). Também os textos que o
  programa grava (log de renderização, metadados de WebM/Lottie, nome da composição de
  demonstração, preset de atalhos "Efeitos Default").
- **Marca ArtCraft removida.** O NOTICE do EffectCraft diz que o nome, o logotipo e as marcas
  ArtCraft (`docs/brand/`) são marcas registradas fora da licença de código aberto. Por isso:
  `docs/brand/` foi apagado, e o símbolo ArtCraft que aparecia na barra de ferramentas e na janela
  "Sobre" foi trocado pelo ícone do Efeitos (`assets/fisilab/fisilab-efeitos-128.png`).
- **Ícone:** executável Windows (`assets/fisilab/fisilab-efeitos.ico`) e ícone da janela/barra de
  tarefas (`assets/fisilab/fisilab-efeitos-256.png`, 512 px no macOS).
- **Recursos do executável Windows** (`apps/effectcraft/build.rs`): ProductName "Efeitos",
  FileDescription "Efeitos — Animação gráfica e composição (Suíte FisiLab)", CompanyName
  "FisiLab — LARFIC/UFRGS", LegalCopyright com o crédito ao EffectCraft, OriginalFilename
  `fisilab-efeitos.exe`, InternalName `fisilab-efeitos`.

### Executáveis, identificador e pastas

- Executáveis: `fisilab-efeitos(.exe)` (app) e `fisilab-efeitos-cli(.exe)` (linha de comando), por meio de
  `[[bin]]` nos pacotes `effectcraft` e `effectcraft-cli` (os nomes dos pacotes Rust não mudaram).
- Identificador do app: `ai.storyteller.effectcraft` → `br.ufrgs.fisilab.efeitos` (app id da
  janela no Wayland, integração AppImage).
- Pastas de dados próprias, para não misturar com uma instalação do EffectCraft:
  `%APPDATA%\FisiLab Efeitos` (macOS `~/Library/Application Support/FisiLab Efeitos`, Linux
  `~/.config/fisilab-efeitos`), cache de disco em `%LOCALAPPDATA%\FisiLab Efeitos\Disk Cache`
  (macOS `~/Library/Caches/FisiLab Efeitos/Disk Cache`, Linux `~/.cache/fisilab-efeitos/disk-cache`),
  modo portátil com `FisiLabEfeitosData` (marcadores `portable.txt` ou `Efeitos.portable`), presets
  em `Documentos/FisiLab Efeitos/Presets`, pasta de auto-salvamento "Efeitos Auto-Save" (ao lado do
  projeto). Nenhuma pasta usa só a palavra genérica "Efeitos".
- Onde "Efeitos" sozinho se confundiria com o substantivo comum, o texto diz "FisiLab Efeitos":
  nome e instruções do servidor MCP, a nota que o programa põe na área de transferência ao copiar
  ("FisiLab Efeitos: 1 keyframe") e o título do aviso de perda do dispositivo gráfico.
- Log: o filtro padrão inclui o alvo `fisilab_efeitos` (os executáveis `fisilab-efeitos` e
  `fisilab-efeitos-cli`), além de `effectcraft`, para que as mensagens `info` do próprio app
  continuem aparecendo.

### Identidade visual da Suíte FisiLab

- **Crate `crates/fisilab-brand`** (compartilhado pela suíte, copiado sem alterações): paleta,
  tela de abertura e janela "Sobre a Suíte FisiLab".
- **Tela de abertura** com a logo do CTA nos primeiros ~3 s (`fisilab_brand::splash::enable()` em
  `apps/effectcraft/src/main.rs`; desligada com `FISILAB_NO_SPLASH=1` e quando o app é controlado
  por `--control`).
- **Janela "Sobre a Suíte FisiLab"**: item **Ajuda ▸ Sobre a Suíte FisiLab…** (comando
  `help.fisilab`), botão "Suíte FisiLab" na barra de ferramentas (no lugar do botão do Discord) e
  links na tela inicial e na janela "Sobre o Efeitos". A janela "Sobre o Efeitos" original,
  com os créditos aos contribuidores do EffectCraft, foi mantida.
- **Paleta original do EffectCraft**, com os temas **"Noite"** (escuro, o antigo "Dark", padrão) e
  **"Dia"** (claro, o antigo "Light") em primeiro lugar na lista (ids `dark` e `light`, as cores
  são exatamente as do EffectCraft; comandos `view.theme.dark`/`view.theme.light`). O tema
  "Darker" continua disponível em Configurações ▸ Aparência.

### Links e comunicação com servidores

- Links de comunidade trocados (Ajuda, tela inicial, barra de ferramentas, janela Sobre):
  "Site da Suíte FisiLab", "Manifesto pelas tecnologias livres", "Projeto original (EffectCraft)"
  (`https://github.com/storytold/effectcraft`) e "Reportar um problema"
  (`https://github.com/mrcandrade/fisilab-efeitos/issues`). Removidos: o Discord do ArtCraft, o site
  getartcraft.com e a lista "Mais apps ArtCraft".
- O EffectCraft não tem verificação de atualizações nem telemetria. Os únicos downloads são os
  modelos opcionais (MobileSAM no GitHub e MediaPipe Face Landmarker no Google), feitos só quando o
  usuário pede, a partir de servidores neutros; foram mantidos.

### Idioma

- Na **primeira execução** (sem configurações salvas) o programa abre em **Português (Brasil)**
  (definido em `apps/effectcraft/src/main.rs`; o padrão da biblioteca continua `system`, como no
  EffectCraft); a escolha do usuário (inclusive "Igual ao sistema") continua funcionando.

### Repositório e documentação

- `.github/` apagado (os workflows de CI, release e financiamento do EffectCraft rodariam contra
  este fork).
- `README.md` novo, em português; o original está em `README.upstream.md`.
- `NOTICE` com um cabeçalho do Efeitos antes do texto original (mantido integralmente).
- Empacotamento Windows (`packaging/windows/`): nomes dos executáveis, do produto e do fabricante,
  novos GUIDs de upgrade/componente (não colidem com o EffectCraft), links e `portable.txt` em
  português. O MSI instala em `Program Files\Suite FisiLab\Efeitos` e guarda a escolha do atalho na
  área de trabalho em `HKLM`/`HKCU\Software\FisiLab\Efeitos`; `test-msi.ps1` testa esses nomes. A suíte distribui um instalador Inno Setup próprio.
- Links do Discord/getartcraft.com trocados em `CLAUDE.md`, `CONTRIBUTING.md`, `ROADMAP.md` e nos
  metadados de pacotes Linux; `docs/preferences.md` documenta os novos padrões.

## O que NÃO foi alterado

- Todo o motor de composição, efeitos, codecs, expressões, scripts e a interface (além das cores,
  dos textos e dos links acima).
- Os nomes dos pacotes, crates e módulos Rust (`effectcraft-*`), os identificadores internos dos
  comandos, as variáveis de ambiente (`EFFECTCRAFT_CONFIG_DIR`, `EFFECTCRAFT_CONTROL_PORT`…) e os
  formatos de arquivo: projetos continuam `.ecproj`/`.ecprojx` (raiz XML `EffectCraftProject`),
  modelos `.ectemplate`, e são compatíveis com o EffectCraft.
- Comentários de código e o histórico da documentação técnica em `docs/`.
- `LICENSE-MIT`, `LICENSE-APACHE` e `ATTRIBUTION.md`.
- O ícone original do EffectCraft (`assets/app-icon/`, MIT OR Apache-2.0) continua no repositório
  (usado em testes e no app web), mas o app desktop usa o ícone do Efeitos.
- O app web (`apps/effectcraft-web`) só teve o nome visível trocado.
