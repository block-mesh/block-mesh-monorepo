import styles from './styles.module.css'

type Props = React.MenuHTMLAttributes<HTMLMenuElement> & {
  current: 'connecting' | 'login' | 'done';
};

export default function MenuMain({ current, ...props }: Props) {
  return (
    <menu className={styles.menu} {...props}>
      <li>
        <a href="/" aria-current={current === 'connecting'}>
          <span>Connect wallet</span>
        </a>
      </li>
      <li>
        <a href="/login" aria-current={current === 'login'}>
          <span>Login</span>
        </a>
      </li>
      <li>
        <a href="/done" aria-current={current === 'done'}>
          <span>Done</span>
        </a>
      </li>
    </menu>
  )
}