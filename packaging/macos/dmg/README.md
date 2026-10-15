# Janela do DMG

Dois arquivos, copiados por `package.sh` para dentro da imagem:

| Arquivo | O que é |
|---|---|
| `background.tiff` | Fundo da janela (HiDPI, 1x + 2x), copiado para `.background/background.tiff`. |
| `dmg-layout.DS_Store` | Ajustes do Finder para o volume (tamanho da janela, posição dos ícones, fundo); vai para `.DS_Store`. |

Ambos são gerados por `suite-fisilab/scripts/gerar_mac.py`, em https://github.com/mrcandrade/larfic. Não edite à mão.

O layout espera:

- volume chamado **`Efeitos`** (exatamente, sem a versão: o Finder acha o fundo pelo nome do volume);
- o aplicativo em **`Efeitos.app`**;
- o atalho para `/Applications` chamado **`Aplicativos`**.
