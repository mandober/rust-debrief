# LC Examples

**Example 1**

lazy evaluation:

$$
(\lambda x.x)(\ (\lambda y.(\lambda z.z))(\lambda x.x)\ )\\
(\lambda y.(\lambda z.z))(\lambda x.x)\\
\lambda z.z
$$

eager evaluation:

$$
(\lambda x.x)(\ (\lambda y.(\lambda z.z))(\lambda x.x)\ )\\
(\lambda x.x)(\lambda z.z)\\
\lambda z.z
$$


**Example 2**

$$
(\lambda f.ff)(\lambda f.ff)\\
(\lambda f.ff)(\lambda f.ff)\\
\dots
$$


**Example 2**

$$
(\lambda f.fff)(\lambda f.fff)\\
(\lambda f.fff)(\lambda f.fff)(\lambda f.fff)\\
(\ (\lambda f.fff)(\lambda f.fff)\ )\ (\lambda f.fff)\\
(\ (\lambda f.fff)(\lambda f.fff)(\lambda f.fff)\ )\ (\lambda f.fff)\\
\dots
$$
