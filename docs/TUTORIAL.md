# Tutorial: Snov.io Screenshot tool

Guia de cada ferramenta: o que ela faz, como usar e um atalho de teclado quando existe.

> **Imagens pendentes:** os espaços marcados com `[imagem: ...]` precisam de uma captura real do app. O nome sugerido de cada arquivo está entre parênteses e deve ir para `docs/images/`.

## Como começar uma captura

1. Pressione **Shift + Backspace** (ou o atalho escolhido em Settings). A tela escurece e congela.
2. Clique e arraste para selecionar a área. Ao soltar, aparecem a barra vertical de ferramentas (à direita da área) e a barra horizontal (embaixo).
3. A etiqueta acima da área mostra o tamanho em pixels.

`[imagem: seleção com etiqueta de tamanho e as duas barras (01-selecao.png)]`

Para recomeçar, pressione **Esc** ou clique no **X** da barra horizontal. Clicar fora da área não cancela, para você não perder o recorte por engano.

## Barra vertical: ferramentas

De cima para baixo:

### 1. Borracha (Limpar tudo)

Apaga **todas** as anotações da captura de uma vez. Não apaga a seleção nem a imagem. Pode ser desfeita com **Ctrl + Z**.

`[imagem: antes e depois da borracha (02-borracha.png)]`

### 2. Mover (cruz)

Arrasta a **área selecionada** inteira pela tela, sem mexer nas anotações. Use para reposicionar o recorte.

- Com a cruz ativa, arrastar dentro da área move a área.
- Com outra ferramenta, arrastar dentro da área desenha.
- Para mudar o tamanho, arraste os quadradinhos nas bordas e cantos.

`[imagem: área sendo movida com a cruz (03-mover.png)]`

### 3. Mão livre (caneta)

Desenho livre, como um marcador. Use para rabiscar, circular algo ou sublinhar.

- Atalho: **P**
- Traço: espessura escolhida em Settings ou com a roda do mouse.

`[imagem: traço livre em volta de um botão (04-mao-livre.png)]`

### 4. Linha

Linha reta entre dois pontos. Segure **Shift** para travar em ângulos de 45°.

- Atalho: **L**

`[imagem: linha apontando um campo (05-linha.png)]`

### 5. Seta

Seta com ponta, para indicar algo. Segure **Shift** para travar em 45°.

- Atalho: **A**

`[imagem: seta apontando para um elemento da tela (06-seta.png)]`

### 6. Retângulo

Contorno retangular para destacar uma área. Segure **Shift** para fazer um quadrado.

- Atalho: **R**

`[imagem: retângulo em volta de um bloco de texto (07-retangulo.png)]`

### 7. Elipse

Contorno oval. Segure **Shift** para fazer um círculo.

- Atalho: **E**

`[imagem: elipse em volta de um ícone (08-elipse.png)]`

### 8. Marca-texto

Faixa translúcida, como um marca-texto de papel. Não cobre o conteúdo por completo, então dá para ler o que está embaixo.

- Atalho: **M**

`[imagem: trecho de texto marcado em amarelo (09-marca-texto.png)]`

### 9. Texto

Caixa para escrever. Clique na área onde quer o texto e digite.

- Atalho: **T**
- **Enter** confirma. **Shift + Enter** quebra a linha. Clicar fora da caixa também confirma.
- O texto fica num balão branco com borda roxa, de cantos retos.
- Tamanho: a roda do mouse muda o tamanho enquanto você digita.
- Fonte: Inter, embutida no app.

`[imagem: balão de texto na captura (10-texto.png)]`

#### Mover um texto já colocado

Depois de confirmar um texto, dá para movê-lo sem refazer:

1. Passe o mouse sobre o **T** da barra vertical. Aparece um ícone de cruz do lado de fora da barra.
2. Clique na cruz. A ferramenta de texto continua destacada.
3. Arraste o texto. Ele fica sempre dentro da área selecionada.
4. Clique no **T** para voltar a escrever.

`[imagem: ícone de mover texto aparecendo ao lado do T (11-mover-texto.png)]`

### 10. Pixelar

Borra a região escolhida com um efeito de blocos, para esconder informações sensíveis (nomes, e-mails, números).

- Atalho: **B**

`[imagem: e-mail pixelado (12-pixelar.png)]`

### 11. Número (contador)

Coloca marcadores numerados (1, 2, 3…) para explicar passos. A numeração se ajusta sozinha quando você apaga um número.

- Atalho: **N**
- Cores disponíveis: vermelho, verde, azul e roxo.

`[imagem: passos 1, 2 e 3 numerados (13-numero.png)]`

### 12. Cor

A bolinha colorida abre a paleta de cores, à esquerda da barra. Cores: amarelo, azul, verde, vermelho, preto, branco e roxo (Snov.io).

- Atalhos **1 a 6** escolhem as seis primeiras cores.
- A cor escolhida vale para as próximas anotações.

`[imagem: paleta aberta ao lado da barra (14-cor.png)]`

### 13. Desfazer

Desfaz a última ação. Atalho: **Ctrl + Z**. Para refazer, use **Ctrl + Y**.

`[imagem: seta de desfazer (15-desfazer.png)]`

## Barra horizontal: ações

### Copiar

Copia a captura, com as anotações, para a área de transferência. Depois é só colar (**Ctrl + V**) em qualquer lugar. A captura fecha em seguida.

- Atalho: **Ctrl + C**

### Salvar

Abre a janela "Salvar como" com o formato padrão das configurações (JPEG ou PNG). A extensão que você digitar no nome define o formato final. A captura fecha depois de salvar.

- Atalho: **Ctrl + S**

`[imagem: janela Salvar como (16-salvar.png)]`

### Fechar (X)

Descarta a captura sem copiar nem salvar.

- Atalho: **Esc**

## Apagar

- **Um texto:** use o ícone de mover texto (ao lado do T), clique no texto e pressione **Delete**.
- **Tudo de uma vez:** use a borracha (primeiro botão da barra vertical).
- **Último passo:** **Ctrl + Z** desfaz a última ação.

## Dicas

- **Mover a seleção sem mexer nos desenhos:** use a cruz.
- **Desenhar com precisão:** segure **Shift** nas ferramentas de linha, seta, retângulo e elipse.
- **Não perder um recorte:** clicar fora da área não cancela a seleção.
- **Atalhos das ferramentas** só funcionam enquanto você não está digitando um texto.
