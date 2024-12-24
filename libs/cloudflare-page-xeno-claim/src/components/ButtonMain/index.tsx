import styles from "./styles.module.css";

type Props = React.ButtonHTMLAttributes<HTMLButtonElement>;

export default function ButtonMain({ children, ...props }: Props) {
  return (
    <button className={styles.button} type="submit" {...props}>
      {children}
    </button>
  );
}
