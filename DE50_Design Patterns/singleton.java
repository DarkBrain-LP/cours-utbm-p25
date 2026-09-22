class Singleton{
    private static Singleton instance;


    public Singleton(){
        if (instance != null) {
            instance = new Singleton();
        }
    } 

    public static Singleton getInstance() {
        if (instance == null) {
            instance = new Singleton();
        }
        return instance;
    }

    // Example method
    public void showMessage() {
        System.out.println("Hello from Singleton!");
    }
}