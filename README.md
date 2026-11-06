# Snov.io - Screenshot tool

![Snov.io Screenshot tool em uso](assets/preview.png)

Capturador de tela para Windows, leve e sem rede. Você seleciona uma área, anota e copia ou salva a imagem. Fica na bandeja do sistema e não grava nada em disco sem você pedir.

📖 **Tutorial completo, ferramenta por ferramenta:** [docs/TUTORIAL.md](docs/TUTORIAL.md)

## Download

1. Acesse a página de [Releases](../../releases) e baixe o arquivo `Snovio-Screenshot-Tool.exe` da versão mais recente.
2. Coloque o arquivo em uma pasta de sua escolha (ele guarda o `settings.json` ao lado dele) e dê dois cliques para abrir.
3. O app aparece na bandeja, perto do relógio.

> Se o Windows mostrar "O Windows protegeu o seu PC" (SmartScreen), clique em **Mais informações** e depois em **Executar assim mesmo**. Isso acontece com programas sem assinatura digital.

Não precisa instalar nada. Para parar o app, clique com o botão direito no ícone da bandeja → **Exit**.

## Uso rápido

| O que fazer | Como |
|---|---|
| Começar uma captura | **Shift + Backspace** (ou o atalho que você escolher em Settings) |
| Selecionar a área | Arraste com o mouse e solte |
| Anotar | Escolha uma ferramenta na barra vertical ao lado da seleção |
| Copiar | Botão de copiar na barra horizontal, ou **Ctrl + C** |
| Salvar em arquivo | Botão de salvar, ou **Ctrl + S** |
| Cancelar | **Esc** ou o botão X da barra horizontal |

Cada ferramenta, com exemplos, está no [tutorial](docs/TUTORIAL.md).

## Configurações

Clique com o botão direito no ícone da bandeja → **Settings**. Lá você escolhe:

- **Capture hotkey:** Shift+BKSP, Shift+Insert, Shift+PrtScn ou **Custom**. Em Custom, clique no campo e pressione a combinação (precisa ter Ctrl, Alt ou Shift, ou ser uma tecla F).
- **Default color:** cor que a ferramenta começa usando. O padrão é o roxo Snov.io.
- **Stroke width** e **Text size:** espessura do traço e tamanho do texto, com prévia.
- **Save format:** JPEG (padrão) ou PNG.
- **Toolbar style:** *Conservative (light)*, a barra clara, ou *Modern (dark)*, a barra escura.
- **Start with Windows:** abre o app junto com o Windows.

As mudanças valem na próxima captura. Se o atalho escolhido já estiver em uso por outro programa, o app usa Print Screen e avisa.

## Para quem for compilar

Requisitos: [Rust](https://rustup.rs) (toolchain estável), Windows 10 ou 11.

```
cargo build --release          # gera target\release\Screenshotter.exe
cargo test                     # roda os testes
```

A fonte Inter vai embutida no executável (`assets/fonts/`, licença OFL em `LICENSE-Inter.txt`).

## Licença

Código e marca: uso interno da Snov.io. A fonte Inter segue a licença SIL Open Font License.
