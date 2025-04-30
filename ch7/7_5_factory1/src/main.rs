// 팩톨리 메서드 패턴
// 팩토리 메서드 패턴은 객체 생성의 인터페이스를 정의하지만, 어떤 클래스의 인스턴스를 만들지는 서브클래스에서 결정하는 패턴이다.

// 이 trait을 구현하는 구조체는 eat() 메서드를 가져야 한다.
trait Pizza {
    fn eat(&self);
}

// 피자 종류 enum 설정
enum PizzaType {
    Bulgogi,
    Hawaiian,
}

// 불고기 피자와 하와이안 피자 구조체 설정
struct BulgogiPizza;
struct Hawaiianpizza;

// 불고기 피자와 하와이안 피자에 대해 Pizza 트레잇을 구현합니다.
impl Pizza for BulgogiPizza {
    fn eat(&self) {
        println!("불고기 냠냠");
    }
}

// 하와이안 피자에 대해 Pizza 트레잇을 구현합니다.
impl Pizza for Hawaiianpizza {
    fn eat(&self) {
        println!("파인애플 냠냠");
    }
}

// PizzaFactory trait을 정의합니다.
// 이 trait을 구현하는 구조체는 create() 메서드를 가져야 합니다.
trait PizzaFactory {
    fn create(pizza_type: PizzaType) -> Box<dyn Pizza>;
}

// ConcretePizzaFactory 구조체를 정의합니다.
struct ConcretePizzaFactory;

// ConcretePizzaFactory에 대해 PizzaFactory 트레잇을 구현합니다.
// 이 구조체는 create() 메서드를 구현하여 피자 객체를 생성합니다.
// create() 메서드는 PizzaType을 인자로 받아 해당하는 피자 객체를 생성합니다.
// 반환 타입은 Box<dyn Pizza>입니다.
// Box<dyn Pizza>는 동적 디스패치를 사용하여 런타임에 어떤 피자 객체인지 결정합니다.
// Box는 힙에 할당된 객체를 가리키는 스마트 포인터입니다.
impl PizzaFactory for ConcretePizzaFactory {
    fn create(pizza_type: PizzaType) -> Box<dyn Pizza> {
        match pizza_type {
            PizzaType::Bulgogi => Box::new(BulgogiPizza),
            PizzaType::Hawaiian => Box::new(Hawaiianpizza),
        }
    }
}

// main 함수에서 ConcretePizzaFactory를 사용하여 피자를 생성하고 먹습니다.
// ConcretePizzaFactory::create() 메서드를 호출하여 피자 객체를 생성합니다.
// 생성된 피자 객체는 Box<dyn Pizza> 타입으로 반환됩니다.
// 이 객체는 eat() 메서드를 호출하여 피자를 먹습니다.
// 이 메서드는 각 피자 객체에 대해 다르게 구현되어 있습니다.
// 따라서 런타임에 어떤 피자 객체인지에 따라 다른 동작을 수행합니다.
fn main() {
    let bulgogi = ConcretePizzaFactory::create(PizzaType::Bulgogi);
    let hawaiian = ConcretePizzaFactory::create(PizzaType::Hawaiian);

    bulgogi.eat();
    hawaiian.eat();
}

/*실행결과
불고기 냠냠
파인애플 냠냠 
*/