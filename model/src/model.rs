// all traits should be adjectives

// we want to cast a wide net, but there are 2 senses 
// we should probably preclude, i.e. Taste & Smell


trait Viewable {
    fn see_starts();
    fn see_during();
    fn see_ends();
}

trait Hearable {
    fn hear_starts();
    fn hear_during();
    fn hear_ends();
}

trait Touchable {
    fn touch_starts();
    fn touch_during();
    fn touch_ends();
}

// now for other common interactions in VTTs

// There is information to be read about it
trait Describable {
    fn read_starts();
    fn read_during();
    fn read_ends();
}

// it can be touched
trait Moveable {
    fn move_starts();
    fn move_during();
    fn move_ends();
}

// it can perform some action
trait Actionable {
    fn action_start();
    fn action_during();
    fn action_end();
}

// it implements some kind of rule
trait Imposable {
}




fn main() {

}
