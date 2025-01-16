import styles from './styles.module.css'

export default function HeaderMain() {
  return (
    <header className={styles.header}>
      <img src={'https://landing-page-assets.blockmesh.xyz/logo-symbol.svg'} className={'w-20 h-20'} />
    </header>
  )
}
