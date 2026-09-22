
class ParentClass{

}

class SubClass1 extends ParentClass{

}
class SubClass2 extends ParentClass{

}

interface Creator {
    ParentClass factoryMethod(String type);
}

class ConcreteCreator implements Creator {
    @Override
    public ParentClass factoryMethod(String type) {
        if (type.equals("SubClass1")) {
            return new SubClass1();
        } else if (type.equals("SubClass2")) {
            return new SubClass2();
        } else {
            return null;
        }
    }
}

public class factoryMethod {
    public static void main(String[] args) {
        Creator creator = new ConcreteCreator();
        ParentClass obj1 = creator.factoryMethod("SubClass1");
        ParentClass obj2 = creator.factoryMethod("SubClass2");

        System.out.println("Created objects: " + obj1.getClass().getSimpleName() + ", " + obj2.getClass().getSimpleName());
    }
}