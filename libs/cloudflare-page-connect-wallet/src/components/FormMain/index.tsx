import styles from './styles.module.css'

type Props = React.FormHTMLAttributes<HTMLFormElement> & {
  'data-current-item'?: 'connecting';
};

export default function FormMain({ children, ...props }: Props) {
  return (
    // @ts-ignore
    <div className={styles.form} {...props}>
      {children}
    </div>
  )
}
