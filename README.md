# qit

> *Overly opinionated git tooling.*

qit is a utility that wraps around git in order to make some operations less
painful.

## Pretty pictures

![](https://cdn.mewna.xyz/2021/12/13/L7JaBjnssyViT.png)

![](https://cdn.mewna.xyz/2021/12/13/ojAq3g41ROiYw.png)

![](https://cdn.mewna.xyz/2021/12/13/EGTgy0UFXDHJ8.png)

## Configuration

qit believes in minimising configuration / being overly opinionated, so only a
bare minimum of configurability is exposed.

Configuration is done via environment variables.

```
# Disable emojis in commit messages
QIT_DISABLE_EMOJIS=true
```