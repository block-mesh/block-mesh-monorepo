import ImgLogoBlockmesh from '../SVGLogoBlockmesh'
import styles from './styles.module.css'

export default function HeaderMain() {
  return (
    <header className={styles.header}>
      <a href={'https://blockmesh.xyz'} target={'_blank'}><ImgLogoBlockmesh /></a>
    </header>
  )
}
