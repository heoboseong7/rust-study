/**
 * 사용 방법
 * <pre>
 * @code Animal.CAT.say()
 *
 * </pre>
 */
public enum Animal {
    DOG {
        @Override
        public void say() {
            System.out.println("멍멍");
        }
    },
    CAT {
        @Override
        public void say() {
            System.out.println("야옹");
        }
    };

    public abstract void say();
}
