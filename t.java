
class t {
	public static int fibonacci(int i) {
		if (i < 2)
			return i;
		return fibonacci(i - 2) + fibonacci(i - 1);
	}
	public static void main(String[] arg) {
		System.out.println(fibonacci(45));
	}
}
